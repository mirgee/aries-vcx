use crate::protocols::did_exchange::states::{
    completed::Completed, requester::request_sent::RequestSent, responder::response_sent::ResponseSent,
};

use super::{requester::DidExchangeServiceRequester, responder::DidExchangeServiceResponder};

#[derive(Debug, Clone)]
pub enum GenericDidExchange {
    Requester(RequesterState),
    Responder(ResponderState),
}

#[derive(Debug, Clone)]
pub enum RequesterState {
    RequestSent(DidExchangeServiceRequester<RequestSent>),
    Completed(DidExchangeServiceRequester<Completed>),
}

#[derive(Debug, Clone)]
pub enum ResponderState {
    ResponseSent(DidExchangeServiceResponder<ResponseSent>),
    Completed(DidExchangeServiceResponder<Completed>),
}

impl From<DidExchangeServiceRequester<RequestSent>> for GenericDidExchange {
    fn from(state: DidExchangeServiceRequester<RequestSent>) -> Self {
        Self::Requester(RequesterState::RequestSent(state))
    }
}

impl From<DidExchangeServiceRequester<Completed>> for GenericDidExchange {
    fn from(state: DidExchangeServiceRequester<Completed>) -> Self {
        Self::Requester(RequesterState::Completed(state))
    }
}

impl From<DidExchangeServiceResponder<ResponseSent>> for GenericDidExchange {
    fn from(state: DidExchangeServiceResponder<ResponseSent>) -> Self {
        Self::Responder(ResponderState::ResponseSent(state))
    }
}

impl From<DidExchangeServiceResponder<Completed>> for GenericDidExchange {
    fn from(state: DidExchangeServiceResponder<Completed>) -> Self {
        Self::Responder(ResponderState::Completed(state))
    }
}
