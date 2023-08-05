use std::{default, sync::Arc};

use aries_vcx_core::{ledger::base_ledger::IndyLedgerRead, wallet::base_wallet::BaseWallet};
use did_doc::schema::verification_method::{VerificationMethod, VerificationMethodType};
use did_doc_sov::{
    extra_fields::{didcommv1::ExtraFieldsDidCommV1, didcommv2::ExtraFieldsDidCommV2, KeyKind},
    service::{aip1::ServiceAIP1, didcommv1::ServiceDidCommV1, didcommv2::ServiceDidCommV2, ServiceSov},
    DidDocumentSov,
};
use did_key::DidKey;
use did_parser::Did;
use did_peer::{
    peer_did::{generate::generate_numalgo2, numalgos::numalgo2::Numalgo2, peer_did::PeerDid},
    peer_did_resolver::resolver::PeerDidResolver,
};
use did_resolver::traits::resolvable::DidResolvable;
use diddoc_legacy::aries::service::AriesService;
use messages::{
    decorators::thread::ThreadGoalCode,
    msg_fields::protocols::{
        did_exchange::{complete::Complete, request::Request, response::Response},
        out_of_band::{
            invitation::{Invitation as OobInvitation, OobService},
            OobGoalCode,
        },
    },
};
use public_key::{Key, KeyType};
use shared_vcx::maybe_known::MaybeKnown;
use url::Url;

use crate::{
    common::{
        keys::get_verkey_from_ledger,
        ledger::transactions::{into_did_doc, resolve_service},
    },
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    handlers::util::AnyInvitation,
    protocols::{
        did_exchange::{
            helpers::attach_to_ddo_sov,
            initiation_type::Requester,
            protocol::requester::{DidExchangeRequestParams, DidExchangeRequester},
            record::ConnectionRecord,
            states::{completed::Completed, requester::request_sent::RequestSent},
            transition::transition_result::TransitionResult,
        },
        mediated_connection::pairwise_info::PairwiseInfo,
    },
    utils::{from_legacy_did_doc_to_sov, from_legacy_service_to_service_sov},
};

use super::{construct_service, did_doc_from_keys, generate_keypair, DidExchangeService};

pub type DidExchangeServiceRequester<S> = DidExchangeService<Requester, S>;

fn map_goal_code(oob_goal_code: MaybeKnown<OobGoalCode>) -> MaybeKnown<ThreadGoalCode> {
    match oob_goal_code {
        MaybeKnown::Known(goal_code) => match goal_code {
            OobGoalCode::IssueVC => MaybeKnown::Known(ThreadGoalCode::AriesVcIssue),
            OobGoalCode::RequestProof => MaybeKnown::Known(ThreadGoalCode::AriesVcVerify),
            OobGoalCode::CreateAccount | OobGoalCode::P2PMessaging => MaybeKnown::Known(ThreadGoalCode::AriesRelBuild),
        },
        MaybeKnown::Unknown(goal_code) => MaybeKnown::Unknown(goal_code),
    }
}

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

async fn create_our_did_document(
    wallet: &Arc<dyn BaseWallet>,
    service_endpoint: Url,
    routing_keys: Vec<String>,
) -> Result<(DidDocumentSov, Key), AriesVcxError> {
    let key_ver = generate_keypair(wallet, KeyType::Ed25519).await?;
    let key_enc = generate_keypair(wallet, KeyType::X25519).await?;
    let service = construct_service(
        routing_keys.into_iter().map(KeyKind::Value).collect(),
        vec![KeyKind::DidKey(key_enc.clone().try_into().unwrap())],
        service_endpoint,
    )?;
    Ok((
        did_doc_from_keys(Default::default(), key_ver, key_enc.clone(), service),
        key_enc,
    ))
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
        let our_peer_did = generate_numalgo2(our_did_document.clone().into())?;
        let params = DidExchangeRequestParams {
            invitation_id: invitation.id,
            label: "".to_string(),
            // Must be non-empty for some reason
            goal: Some("To establish a connection".to_string()),
            goal_code: Some(MaybeKnown::Known(ThreadGoalCode::AriesRelBuild)),
            did: our_peer_did.clone().into(),
            did_doc: Some(our_did_document),
        };
        let TransitionResult {
            state: sm,
            output: request,
        } = DidExchangeRequester::construct_request(params)?;
        Ok(TransitionResult {
            state: Self {
                sm,
                our_verkey,
                their_did_document,
            },
            output: request,
        })
    }

    async fn construct_request_public(
        ledger: Arc<dyn IndyLedgerRead>,
        PublicConstructRequestConfig { their_did, our_did }: PublicConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        let (their_did_document, service) = their_did_doc_from_did(&ledger, their_did.clone()).await?;
        let params = DidExchangeRequestParams {
            invitation_id: format!("{}#{}", their_did, service.id().to_string()),
            label: "".to_string(),
            goal: Some("To establish a connection".to_string()),
            goal_code: Some(MaybeKnown::Known(ThreadGoalCode::AriesRelBuild)),
            did: our_did.clone(),
            did_doc: None,
        };
        let TransitionResult {
            state: sm,
            output: request,
        } = DidExchangeRequester::construct_request(params)?;
        Ok(TransitionResult {
            state: Self {
                sm,
                // TODO: Get it from wallet instead
                our_verkey: Key::from_base58(
                    &get_verkey_from_ledger(&ledger, &our_did.id().to_string()).await?,
                    KeyType::X25519,
                )
                .unwrap()
                .clone(),
                their_did_document,
            },
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
    ) -> Result<TransitionResult<DidExchangeServiceRequester<Completed>, Complete>, AriesVcxError> {
        let TransitionResult {
            state: sm,
            output: complete,
        } = self.sm.construct_complete(response.clone())?;
        let did_document = if let Some(ddo) = response.content.did_doc {
            attach_to_ddo_sov(ddo)?
        } else {
            PeerDidResolver::new()
                .resolve(&response.content.did.parse()?, &Default::default())
                .await?
                .did_document()
                .to_owned()
                .into()
        };
        Ok(TransitionResult {
            state: DidExchangeServiceRequester {
                sm,
                our_verkey: self.our_verkey,
                their_did_document: did_document,
            },
            output: complete,
        })
    }
}

impl DidExchangeServiceRequester<Completed> {
    pub fn to_record(self) -> ConnectionRecord {
        ConnectionRecord::from_parts(self.their_did_document, self.our_verkey)
    }
}
