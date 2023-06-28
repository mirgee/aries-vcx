pub mod requester;
pub mod responder;
pub mod trait_bounds;

use self::trait_bounds::ThreadId;

#[derive(Debug, Clone, PartialEq)]
pub struct DidExchangeProtocol<I, S> {
    initiation_type: I,
    state: S,
}

impl<I, S> DidExchangeProtocol<I, S> {
    pub fn from_parts(initiation_type: I, state: S) -> Self {
        Self { initiation_type, state }
    }

    pub fn into_parts(self) -> (I, S) {
        let Self { initiation_type, state } = self;
        (initiation_type, state)
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}

// TODO: There is very little benefit in having a single struct for both roles if
// no methods are shared between them. Consider splitting this into two structs.
impl<I, S> DidExchangeProtocol<I, S>
where
    S: ThreadId,
{
    pub fn thread_id(&self) -> &str {
        self.state.thread_id()
    }
}

#[cfg(test)]
mod tests {

    use crate::protocols::did_exchange::transition::transition_result::TransitionResult;

    use super::{
        requester::{DidExchangeRequestParams, DidExchangeRequester},
        responder::{DidExchangeResponder, DidExchangeResponseParams},
    };

    use did_doc_sov::DidDocumentSov;
    use did_peer::peer_did::{numalgos::numalgo2::Numalgo2, peer_did::PeerDid};
    use messages::msg_fields::protocols::did_exchange::request::Request;
    use uuid::Uuid;

    fn request_params(invitation_id: String) -> DidExchangeRequestParams {
        DidExchangeRequestParams {
            did: PeerDid::<Numalgo2>::parse("did:peer:2.123".to_string()).unwrap().into(),
            invitation_id,
            label: "test".to_string(),
            did_doc: None,
            goal: None,
            goal_code: None,
        }
    }

    fn response_params(request: Request) -> DidExchangeResponseParams {
        let invitation_id = request.decorators.thread.clone().unwrap().thid;
        let peer_did = PeerDid::<Numalgo2>::parse("did:peer:1.123".to_string()).unwrap();
        DidExchangeResponseParams {
            invitation_id,
            request,
            did: peer_did.clone().into(),
            did_doc: Some(DidDocumentSov::builder(peer_did.into()).build()),
        }
    }

    #[test]
    fn test_did_exchange() {
        let invitation_id = Uuid::new_v4().to_string();
        let TransitionResult {
            state: request_sent_state,
            output: request,
        } = DidExchangeRequester::construct_request(request_params(invitation_id)).unwrap();
        let TransitionResult {
            state: response_sent_state,
            output: response,
        } = DidExchangeResponder::construct_response(response_params(request)).unwrap();
        let TransitionResult {
            state: _requester_completed,
            output: complete,
        } = request_sent_state.construct_complete(response).unwrap();
        let _responder_completed = response_sent_state.receive_complete(complete).unwrap();
    }
}
