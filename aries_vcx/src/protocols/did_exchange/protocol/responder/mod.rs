use did_doc_sov::DidDocumentSov;
use did_parser::Did;
use messages::{
    decorators::thread::Thread,
    msg_fields::protocols::did_exchange::{
        complete::Complete,
        request::Request,
        response::{Response, ResponseContent, ResponseDecorators},
    },
};

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    protocols::did_exchange::{
        helpers::ddo_sov_to_attach,
        initiation_type::Responder,
        states::{completed::Completed, responder::response_sent::ResponseSent},
        transition::{
            transition_error::TransitionError, transition_result::TransitionResult as TransitionResultVerbose,
        },
    },
};

use super::{
    trait_bounds::{InvitationId, RequestId},
    DidExchangeProtocol,
};

pub type DidExchangeResponder<S> = DidExchangeProtocol<Responder, S>;
type TransitionResult<T, U = ()> = TransitionResultVerbose<DidExchangeResponder<T>, U>;

pub struct DidExchangeResponseParams {
    pub request: Request,
    pub did: Did,
    pub did_doc: Option<DidDocumentSov>,
    pub invitation_id: String,
}

impl DidExchangeResponder<ResponseSent> {
    pub fn construct_response(
        DidExchangeResponseParams {
            request,
            did,
            did_doc,
            invitation_id,
        }: DidExchangeResponseParams,
    ) -> Result<TransitionResult<ResponseSent, Response>, AriesVcxError> {
        if request.decorators.thread.and_then(|t| t.pthid) != Some(invitation_id.clone()) {
            return Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidState,
                "Parent thread ID of the request does not match the id of the invite",
            ));
        }
        // TODO The DDO must be signed by the pw vk in the recipient keys of the invitation
        // (probably use a new trait for this)
        let content = ResponseContent {
            did: did.to_string(),
            did_doc: did_doc.clone().map(ddo_sov_to_attach),
        };
        let thread = {
            let mut thread = Thread::new(request.id.clone());
            thread.pthid = Some(invitation_id.clone());
            thread
        };
        let decorators = ResponseDecorators { thread, timing: None };
        let response = Response::with_decorators(request.id.clone(), content, decorators);
        Ok(TransitionResult {
            state: DidExchangeResponder::from_parts(
                Responder,
                ResponseSent {
                    invitation_id,
                    request_id: request.id,
                },
            ),
            output: response,
        })
    }

    pub fn receive_complete(
        self,
        complete: Complete,
    ) -> Result<DidExchangeResponder<Completed>, TransitionError<Self>> {
        if complete.decorators.thread.thid != self.request_id() {
            return Err(TransitionError {
                state: self,
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Thread ID of the Complete message deos not match thread ID of the conversation",
                ),
            });
        }
        if complete.decorators.thread.pthid != Some(self.invitation_id().to_string()) {
            return Err(TransitionError {
                state: self,
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Parent thread ID of the Complete message deos not match thread ID of the invitation",
                ),
            });
        }
        Ok(DidExchangeResponder::from_parts(Responder, Completed))
    }
}

impl RequestId for DidExchangeResponder<ResponseSent> {
    fn request_id(&self) -> &str {
        self.state.request_id.as_str()
    }
}

impl InvitationId for DidExchangeResponder<ResponseSent> {
    fn invitation_id(&self) -> &str {
        self.state.invitation_id.as_str()
    }
}
