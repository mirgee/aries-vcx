use did_doc_sov::DidDocumentSov;
use did_parser::Did;
use messages::{
    decorators::thread::{Thread, ThreadGoalCode},
    msg_fields::protocols::did_exchange::{
        complete::{Complete as CompleteMessage, CompleteDecorators},
        request::{Request, RequestContent, RequestDecorators},
        response::Response,
    },
};
use shared_vcx::{maybe_known::MaybeKnown, misc::serde_ignored::SerdeIgnored as NoContent};
use uuid::Uuid;

use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind},
    protocols::did_exchange::{
        helpers::ddo_sov_to_attach,
        initiation_type::Requester,
        states::{completed::Completed, requester::request_sent::RequestSent},
        transition::{
            transition_error::TransitionError, transition_result::TransitionResult as TransitionResultVerbose,
        },
    },
};

use super::{
    trait_bounds::{InvitationId, RequestId},
    DidExchangeProtocol,
};

pub type DidExchangeRequester<S> = DidExchangeProtocol<Requester, S>;
type TransitionResult<T, U> = TransitionResultVerbose<DidExchangeRequester<T>, U>;

#[derive(Debug, Clone)]
pub struct DidExchangeRequestParams {
    pub invitation_id: String,
    pub label: String,
    pub goal: Option<String>,
    pub goal_code: Option<MaybeKnown<ThreadGoalCode>>,
    // TODO: Provide just a single field - did or did doc
    pub did: Did,
    pub did_doc: Option<DidDocumentSov>,
}

impl From<DidExchangeRequestParams> for RequestContent {
    fn from(params: DidExchangeRequestParams) -> Self {
        let DidExchangeRequestParams {
            label,
            goal,
            goal_code,
            did,
            did_doc,
            ..
        } = params;
        RequestContent {
            label,
            goal_code,
            goal,
            did: did.to_string(),
            did_doc: did_doc.map(ddo_sov_to_attach),
        }
    }
}

impl DidExchangeRequester<RequestSent> {
    pub fn construct_request(
        params: DidExchangeRequestParams,
    ) -> Result<TransitionResult<RequestSent, Request>, TransitionError<Self>> {
        let request_id = Uuid::new_v4().to_string();
        let invitation_id = params.invitation_id.clone();
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
        let request = Request::with_decorators(request_id.clone(), params.clone().into(), decorators);
        // TODO: If the provided DID is not resolvable, we need to add DDO (signed with a keyAgreement key?)
        // But the request is sent encrypted!
        Ok(TransitionResult {
            state: DidExchangeRequester::from_parts(
                Requester,
                RequestSent {
                    request_id,
                    invitation_id,
                },
            ),
            output: request,
        })
    }

    pub fn construct_complete(
        self,
        response: Response,
    ) -> Result<TransitionResult<Completed, CompleteMessage>, TransitionError<Self>> {
        if response.decorators.thread.thid != self.request_id() {
            return Err(TransitionError {
                state: self,
                error: AriesVcxError::from_msg(
                    AriesVcxErrorKind::InvalidState,
                    "Thread ID of the response does not match ID of the request",
                ),
            });
        }
        let complete_id = Uuid::new_v4().to_string();
        let decorators = {
            let mut thread = Thread::new(self.request_id().to_string());
            thread.pthid = Some(self.invitation_id().to_string());
            CompleteDecorators { thread, timing: None }
        };
        let complete_message = CompleteMessage::with_decorators(complete_id, NoContent::default(), decorators);
        Ok(TransitionResult {
            state: DidExchangeRequester::from_parts(Requester, Completed),
            output: complete_message,
        })
    }
}

impl RequestId for DidExchangeRequester<RequestSent> {
    fn request_id(&self) -> &str {
        self.state.request_id.as_str()
    }
}

impl InvitationId for DidExchangeRequester<RequestSent> {
    fn invitation_id(&self) -> &str {
        self.state.invitation_id.as_str()
    }
}
