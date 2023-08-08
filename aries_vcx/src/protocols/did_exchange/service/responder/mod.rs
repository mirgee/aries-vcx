use std::sync::Arc;

use aries_vcx_core::wallet::base_wallet::BaseWallet;
use did_doc_sov::DidDocumentSov;
use did_resolver_registry::ResolverRegistry;
use messages::{
    decorators::thread::Thread,
    msg_fields::protocols::did_exchange::{
        complete::Complete,
        request::Request,
        response::{Response, ResponseContent, ResponseDecorators},
    },
};
use url::Url;

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    protocols::did_exchange::{
        states::{completed::Completed, responder::response_sent::ResponseSent},
        transition::{transition_error::TransitionError, transition_result::TransitionResult},
    },
};

use super::{attach_to_ddo_sov, create_our_did_document, ddo_sov_to_attach, DidExchangeService};

#[derive(Clone, Copy, Debug)]
pub struct Responder;

pub type DidExchangeServiceResponder<S> = DidExchangeService<Responder, S>;

pub struct ReceiveRequestConfig {
    pub wallet: Arc<dyn BaseWallet>,
    pub resolver_registry: Arc<ResolverRegistry>,
    pub request: Request,
    pub service_endpoint: Url,
    pub routing_keys: Vec<String>,
    pub invitation_id: String,
}

async fn resolve_their_ddo(
    resolver_registry: &Arc<ResolverRegistry>,
    request: &Request,
) -> Result<DidDocumentSov, AriesVcxError> {
    if let Some(ddo) = request.content.did_doc.clone().map(attach_to_ddo_sov).transpose()? {
        Ok(ddo)
    } else {
        Ok(resolver_registry
            .resolve(&request.content.did.parse()?, &Default::default())
            .await?
            .did_document()
            .to_owned()
            .into())
    }
}

// TODO: Replace by a builder
fn construct_response(our_did_document: DidDocumentSov, invitation_id: String, request_id: String) -> Response {
    let content = ResponseContent {
        did: our_did_document.id().to_string(),
        did_doc: Some(ddo_sov_to_attach(our_did_document.clone())),
    };
    let thread = {
        let mut thread = Thread::new(request_id.clone());
        thread.pthid = Some(invitation_id.clone());
        thread
    };
    let decorators = ResponseDecorators { thread, timing: None };
    Response::with_decorators(request_id, content, decorators)
}

impl DidExchangeServiceResponder<ResponseSent> {
    pub async fn receive_request(
        ReceiveRequestConfig {
            wallet,
            resolver_registry,
            request,
            service_endpoint,
            routing_keys,
            invitation_id,
        }: ReceiveRequestConfig,
    ) -> Result<TransitionResult<DidExchangeServiceResponder<ResponseSent>, Response>, AriesVcxError> {
        let their_ddo = resolve_their_ddo(&resolver_registry, &request).await?;
        let (our_ddo, enc_key) = create_our_did_document(&wallet, service_endpoint, routing_keys).await?;

        if request.decorators.thread.and_then(|t| t.pthid) != Some(invitation_id.clone()) {
            return Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidState,
                "Parent thread ID of the request does not match the id of the invite",
            ));
        }

        let response = construct_response(our_ddo, invitation_id.clone(), request.id.clone());

        Ok(TransitionResult {
            state: DidExchangeServiceResponder::from_parts(
                ResponseSent {
                    request_id: request.id,
                    invitation_id,
                },
                their_ddo,
                enc_key,
            ),
            output: response,
        })
    }
}

impl DidExchangeServiceResponder<ResponseSent> {
    pub fn receive_complete(
        self,
        complete: Complete,
    ) -> Result<DidExchangeServiceResponder<Completed>, TransitionError<Self>> {
        if complete.decorators.thread.thid != self.state.request_id {
            return Err(TransitionError {
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Thread ID of the complete message does not match the id of the request",
                ),
                state: self,
            });
        }
        if complete.decorators.thread.pthid != Some(self.state.invitation_id.to_string()) {
            return Err(TransitionError {
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Parent thread ID of the complete message does not match the id of the invite",
                ),
                state: self,
            });
        }
        Ok(DidExchangeServiceResponder::from_parts(
            Completed,
            self.their_did_document,
            self.our_verkey,
        ))
    }
}
