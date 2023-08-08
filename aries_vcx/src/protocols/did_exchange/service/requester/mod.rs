use std::sync::Arc;

use aries_vcx_core::{ledger::base_ledger::IndyLedgerRead, wallet::base_wallet::BaseWallet};
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{service::ServiceSov, DidDocumentSov};
use did_parser::{Did, ParseError};
use did_peer::peer_did_resolver::resolver::PeerDidResolver;
use did_resolver::{error::GenericError, traits::resolvable::DidResolvable};
use messages::{
    decorators::thread::{Thread, ThreadGoalCode},
    msg_fields::protocols::{
        did_exchange::{
            complete::{Complete as CompleteMessage, CompleteDecorators},
            request::{Request, RequestContent, RequestDecorators},
            response::Response,
        },
        out_of_band::invitation::{Invitation as OobInvitation, OobService},
    },
};
use public_key::{Key, KeyType};
use shared_vcx::{maybe_known::MaybeKnown, misc::serde_ignored::SerdeIgnored as NoContent};
use url::Url;
use uuid::Uuid;

use crate::{
    common::{
        keys::get_verkey_from_ledger,
        ledger::transactions::{into_did_doc, resolve_service},
    },
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    handlers::util::AnyInvitation,
    protocols::did_exchange::{
        states::{completed::Completed, requester::request_sent::RequestSent},
        transition::{transition_error::TransitionError, transition_result::TransitionResult},
    },
    utils::{from_legacy_did_doc_to_sov, from_legacy_service_to_service_sov},
};

use super::{attach_to_ddo_sov, create_our_did_document, ddo_sov_to_attach, DidExchangeService};

#[derive(Clone, Copy, Debug)]
pub struct Requester;

pub type DidExchangeServiceRequester<S> = DidExchangeService<Requester, S>;

pub struct PairwiseConstructRequestConfig {
    pub invitation: OobInvitation,
    pub wallet: Arc<dyn BaseWallet>,
    pub service_endpoint: Url,
    pub routing_keys: Vec<String>,
}

pub struct PublicConstructRequestConfig {
    pub their_did: Did,
    pub our_did: Did,
}

pub enum ConstructRequestConfig {
    Pairwise(PairwiseConstructRequestConfig),
    Public(PublicConstructRequestConfig),
}

fn verify_handshake_protocol(invitation: OobInvitation) -> Result<(), AriesVcxError> {
    invitation
        .content
        .handshake_protocols
        .unwrap()
        .iter()
        .find(|protocol| match protocol {
            MaybeKnown::Known(protocol) if protocol.to_string().contains("didexchange") => true,
            _ => false,
        })
        .ok_or(AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidState,
            "Invitation does not contain didexchange handshake protocol",
        ))?;
    Ok(())
}

async fn their_did_doc_from_did(
    ledger: &Arc<dyn IndyLedgerRead>,
    their_did: Did,
) -> Result<(DidDocumentSov, ServiceSov), AriesVcxError> {
    let service = resolve_service(ledger, &OobService::Did(their_did.id().to_string())).await?;
    let vm = VerificationMethod::builder(
        their_did.clone().into(),
        their_did.clone(),
        VerificationMethodType::Ed25519VerificationKey2020,
    )
    // TODO: Make it easier to get the first key in base58 (regardless of initial kind) from ServiceSov
    .add_public_key_base58(service.recipient_keys.first().unwrap().clone())
    .build();
    let sov_service = from_legacy_service_to_service_sov(service.clone())?;
    let their_did_document = DidDocumentSov::builder(their_did.clone())
        .add_service(sov_service.clone())
        .add_controller(their_did)
        .add_verification_method(vm)
        .build();
    Ok((their_did_document, sov_service))
}

fn construct_request(invitation_id: String, our_did: String, our_did_document: Option<DidDocumentSov>) -> Request {
    let request_id = Uuid::new_v4().to_string();
    let thread = {
        let mut thread = Thread::new(request_id.clone());
        thread.pthid = Some(invitation_id.clone());
        thread
    };
    let decorators = {
        let mut decorators = RequestDecorators::default();
        decorators.thread = Some(thread);
        decorators
    };
    let content = RequestContent {
        label: "".to_string(),
        // Must be non-empty for some reason, regardless of invite contents
        goal: Some("To establish a connection".to_string()),
        // Must be non-empty for some reason, regardless of invite contents
        goal_code: Some(MaybeKnown::Known(ThreadGoalCode::AriesRelBuild)),
        did: our_did,
        did_doc: our_did_document.map(ddo_sov_to_attach),
    };
    Request::with_decorators(request_id.clone(), content, decorators)
}

fn construct_complete_message(invitation_id: String, request_id: String) -> CompleteMessage {
    let complete_id = Uuid::new_v4().to_string();
    let decorators = {
        let mut thread = Thread::new(request_id);
        thread.pthid = Some(invitation_id);
        CompleteDecorators { thread, timing: None }
    };
    CompleteMessage::with_decorators(complete_id, NoContent::default(), decorators)
}

impl DidExchangeServiceRequester<RequestSent> {
    async fn construct_request_pairwise(
        ledger: Arc<dyn IndyLedgerRead>,
        PairwiseConstructRequestConfig {
            wallet,
            service_endpoint,
            routing_keys,
            invitation,
        }: PairwiseConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        verify_handshake_protocol(invitation.clone())?;
        let (our_did_document, our_verkey) = create_our_did_document(&wallet, service_endpoint, routing_keys).await?;
        let their_did_document =
            from_legacy_did_doc_to_sov(into_did_doc(&ledger, &AnyInvitation::Oob(invitation.clone())).await?)?;

        let request = construct_request(
            invitation.id.clone(),
            our_did_document.id().to_string(),
            Some(our_did_document.clone()),
        );

        Ok(TransitionResult {
            state: DidExchangeServiceRequester::from_parts(
                RequestSent {
                    invitation_id: invitation.id.clone(),
                    request_id: request.id.clone(),
                },
                their_did_document,
                our_verkey,
            ),
            output: request,
        })
    }

    async fn construct_request_public(
        ledger: Arc<dyn IndyLedgerRead>,
        PublicConstructRequestConfig { their_did, our_did }: PublicConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        let (their_did_document, service) = their_did_doc_from_did(&ledger, their_did.clone()).await?;
        let invitation_id = format!("{}#{}", their_did, service.id().to_string());

        let request = construct_request(invitation_id.clone(), our_did.to_string(), None);

        Ok(TransitionResult {
            state: DidExchangeServiceRequester::from_parts(
                RequestSent {
                    request_id: request.id.clone(),
                    invitation_id,
                },
                their_did_document,
                // TODO: Get it from wallet instead
                Key::from_base58(
                    &get_verkey_from_ledger(&ledger, &our_did.id().to_string()).await?,
                    KeyType::X25519,
                )
                .unwrap()
                .clone(),
            ),
            output: request,
        })
    }

    pub async fn construct_request(
        ledger: Arc<dyn IndyLedgerRead>,
        config: ConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        match config {
            ConstructRequestConfig::Pairwise(config) => Self::construct_request_pairwise(ledger, config).await,
            ConstructRequestConfig::Public(config) => Self::construct_request_public(ledger, config).await,
        }
    }

    pub async fn receive_response(
        self,
        response: Response,
    ) -> Result<TransitionResult<DidExchangeServiceRequester<Completed>, CompleteMessage>, TransitionError<Self>> {
        if response.decorators.thread.thid != self.state.request_id {
            return Err(TransitionError {
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Response thread ID does not match request ID",
                ),
                state: self.clone(),
            });
        }
        let did_document = if let Some(ddo) = response.content.did_doc {
            attach_to_ddo_sov(ddo).map_err(|error| TransitionError {
                error,
                state: self.clone(),
            })?
        } else {
            PeerDidResolver::new()
                .resolve(
                    &response
                        .content
                        .did
                        .parse()
                        .map_err(|error: ParseError| TransitionError {
                            error: error.into(),
                            state: self.clone(),
                        })?,
                    &Default::default(),
                )
                .await
                .map_err(|error: GenericError| TransitionError {
                    error: error.into(),
                    state: self.clone(),
                })?
                .did_document()
                .to_owned()
                .into()
        };
        let complete_message =
            construct_complete_message(self.state.invitation_id.clone(), self.state.request_id.clone());
        Ok(TransitionResult {
            state: DidExchangeServiceRequester::from_parts(Completed, did_document, self.our_verkey),
            output: complete_message,
        })
    }
}
