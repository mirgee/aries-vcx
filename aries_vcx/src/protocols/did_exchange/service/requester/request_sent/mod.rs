pub mod config;
mod helpers;

use did_parser::ParseError;
use did_peer::peer_did_resolver::resolver::PeerDidResolver;
use did_resolver::{error::GenericError, traits::resolvable::DidResolvable};
use messages::msg_fields::protocols::did_exchange::{
    complete::Complete as CompleteMessage, request::Request, response::Response,
};
use public_key::{Key, KeyType};

use crate::{
    common::{keys::get_verkey_from_ledger, ledger::transactions::into_did_doc},
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    handlers::util::AnyInvitation,
    protocols::did_exchange::{
        states::{completed::Completed, requester::request_sent::RequestSent},
        transition::{transition_error::TransitionError, transition_result::TransitionResult},
    },
    utils::from_legacy_did_doc_to_sov,
};

use helpers::{construct_complete_message, construct_request, their_did_doc_from_did, verify_handshake_protocol};

use self::config::{ConstructRequestConfig, PairwiseConstructRequestConfig, PublicConstructRequestConfig};

use super::{attach_to_ddo_sov, create_our_did_document, DidExchangeServiceRequester};

impl DidExchangeServiceRequester<RequestSent> {
    async fn construct_request_pairwise(
        PairwiseConstructRequestConfig {
            ledger,
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
        PublicConstructRequestConfig {
            ledger,
            their_did,
            our_did,
        }: PublicConstructRequestConfig,
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
        config: ConstructRequestConfig,
    ) -> Result<TransitionResult<Self, Request>, AriesVcxError> {
        match config {
            ConstructRequestConfig::Pairwise(config) => Self::construct_request_pairwise(config).await,
            ConstructRequestConfig::Public(config) => Self::construct_request_public(config).await,
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
                state: self,
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
