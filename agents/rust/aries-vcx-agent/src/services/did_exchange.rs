use std::sync::Arc;

use aries_vcx::{
    core::profile::profile::Profile,
    did_doc_sov::{
        extra_fields::didcommv2::ExtraFieldsDidCommV2,
        service::{didcommv1::ServiceDidCommV1, didcommv2::ServiceDidCommV2, ServiceSov},
        DidDocumentSov,
    },
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    handlers::out_of_band::sender::OutOfBandSender,
    messages::{
        msg_fields::protocols::{
            did_exchange::{complete::Complete, request::Request, response::Response},
            out_of_band::invitation::{Invitation as OobInvitation, OobService},
        },
        AriesMessage,
    },
    protocols::did_exchange::{
        service::{
            generic::{GenericDidExchange, RequesterState, ResponderState},
            requester::{
                ConstructRequestConfig, DidExchangeServiceRequester, PairwiseConstructRequestConfig,
                PublicConstructRequestConfig,
            },
            responder::DidExchangeServiceResponder,
        },
        states::{requester::request_sent::RequestSent, responder::response_sent::ResponseSent},
        transition::transition_result::TransitionResult,
    },
    transport::Transport,
    utils::{encryption_envelope::EncryptionEnvelope, from_did_doc_sov_to_legacy},
};
use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_resolver_registry::ResolverRegistry;
use uuid::Uuid;

use crate::{
    http_client::HttpClient,
    storage::{object_cache::ObjectCache, Storage},
    AgentError, AgentErrorKind, AgentResult,
};

use super::connection::ServiceEndpoint;

pub struct ServiceDidExchange {
    profile: Arc<dyn Profile>,
    resolver_registry: Arc<ResolverRegistry>,
    service_endpoint: ServiceEndpoint,
    did_exchange: Arc<ObjectCache<GenericDidExchange>>,
    requester_did: String,
}

impl ServiceDidExchange {
    pub fn new(
        profile: Arc<dyn Profile>,
        resolver_registry: Arc<ResolverRegistry>,
        service_endpoint: ServiceEndpoint,
        requester_did: String,
    ) -> Self {
        Self {
            profile,
            service_endpoint,
            resolver_registry,
            did_exchange: Arc::new(ObjectCache::new("did-exchange")),
            requester_did,
        }
    }

    pub async fn send_request_public(&self, their_did: String) -> AgentResult<String> {
        let config = ConstructRequestConfig::Public(PublicConstructRequestConfig {
            their_did: format!("did:sov:{}", their_did).parse()?,
            our_did: format!("did:sov:{}", self.requester_did).parse()?,
        });
        let TransitionResult {
            state: requester,
            output: request,
        } = DidExchangeServiceRequester::<RequestSent>::construct_request(
            self.profile.inject_indy_ledger_read(),
            config,
        )
        .await?;
        let id = self
            .did_exchange
            .insert(&Uuid::new_v4().to_string(), requester.clone().into())?;
        wrap_and_send_msg(
            &self.profile.inject_wallet(),
            &request.clone().into(),
            requester.our_verkey(),
            requester.their_did_doc(),
        )
        .await?;
        Ok(id)
    }

    pub async fn send_request_pairwise(&self, invitation: OobInvitation) -> AgentResult<String> {
        let config = ConstructRequestConfig::Pairwise(PairwiseConstructRequestConfig {
            wallet: self.profile.inject_wallet(),
            invitation: invitation.clone(),
            service_endpoint: self.service_endpoint.clone(),
            routing_keys: vec![],
        });
        let TransitionResult {
            state: requester,
            output: request,
        } = DidExchangeServiceRequester::<RequestSent>::construct_request(
            self.profile.inject_indy_ledger_read(),
            config,
        )
        .await?;
        let id = self.did_exchange.insert(&invitation.id, requester.clone().into())?;
        wrap_and_send_msg(
            &self.profile.inject_wallet(),
            &request.clone().into(),
            requester.our_verkey(),
            requester.their_did_doc(),
        )
        .await?;
        Ok(id)
    }

    pub async fn send_response(&self, request: Request) -> AgentResult<String> {
        let invitation_id = request
            .decorators
            .thread
            .clone()
            .map_or(request.id.clone(), |thread| thread.pthid.unwrap());
        let service = ServiceSov::DIDCommV1(ServiceDidCommV1::new(
            Uuid::new_v4().to_string().parse()?,
            self.service_endpoint.clone().into(),
            Default::default(),
        )?);
        let TransitionResult {
            state: responder,
            output: response,
        } = DidExchangeServiceResponder::<ResponseSent>::receive_request(
            &self.profile.inject_wallet(),
            &self.resolver_registry.clone(),
            request,
            service,
            invitation_id.to_string(),
        )
        .await?;
        let id = self.did_exchange.insert(&response.id, responder.clone().into())?;
        wrap_and_send_msg(
            &self.profile.inject_wallet(),
            &response.clone().into(),
            responder.our_verkey(),
            responder.their_did_doc(),
        )
        .await?;
        Ok(id)
    }

    // TODO: Should it take the thread_id from the response? Prly not
    pub async fn send_complete(&self, thread_id: &str, response: Response) -> AgentResult<()> {
        let TransitionResult {
            state: requester,
            output: complete,
        } = match self.did_exchange.get(thread_id)? {
            GenericDidExchange::Requester(RequesterState::RequestSent(s)) => s.receive_response(response).await?,
            _ => return Err(AgentError::from_kind(AgentErrorKind::InvalidState)),
        };
        self.did_exchange.insert(thread_id, requester.clone().into())?;
        wrap_and_send_msg(
            &self.profile.inject_wallet(),
            &complete.clone().into(),
            requester.our_verkey(),
            requester.their_did_doc(),
        )
        .await?;
        Ok(())
    }

    pub async fn receive_complete(&self, thread_id: &str, complete: Complete) -> AgentResult<()> {
        let sm = match self.did_exchange.get(thread_id)? {
            GenericDidExchange::Responder(ResponderState::ResponseSent(s)) => s.receive_complete(complete)?.into(),
            _ => return Err(AgentError::from_kind(AgentErrorKind::InvalidState)),
        };
        self.did_exchange.insert(thread_id, sm)?;
        Ok(())
    }

    pub fn exists_by_id(&self, thread_id: &str) -> bool {
        self.did_exchange.contains_key(thread_id)
    }
}

pub(crate) async fn wrap_and_send_msg(
    wallet: &Arc<dyn BaseWallet>,
    message: &AriesMessage,
    sender_verkey: &str,
    did_doc: &DidDocumentSov,
) -> VcxResult<()> {
    let env = EncryptionEnvelope::create(
        wallet,
        message,
        Some(sender_verkey),
        &from_did_doc_sov_to_legacy(did_doc.to_owned())?,
    )
    .await?;
    let msg = env.0;
    let service_endpoint = did_doc
        .service()
        .get(0)
        .ok_or_else(|| AriesVcxError::from_msg(AriesVcxErrorKind::InvalidUrl, "No service in DID Doc"))?
        .service_endpoint()
        .clone();

    HttpClient.send_message(msg, service_endpoint.into()).await
}
