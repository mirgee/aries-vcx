use std::sync::Arc;

use aries_vcx_core::ledger::base_ledger::IndyLedgerRead;
use did_doc_sov::DidDocumentSov;
use messages::msg_fields::protocols::did_exchange::{complete::Complete, request::Request, response::Response};
use public_key::Key;

use crate::{
    errors::error::AriesVcxError,
    protocols::did_exchange::{
        states::{completed::Completed, requester::request_sent::RequestSent, responder::response_sent::ResponseSent},
        transition::{transition_error::TransitionError, transition_result::TransitionResult},
    },
};

use super::{
    requester::{ConstructRequestConfig, DidExchangeServiceRequester},
    responder::{DidExchangeServiceResponder, ReceiveRequestConfig},
};

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

impl GenericDidExchange {
    pub fn our_verkey(&self) -> &Key {
        match self {
            GenericDidExchange::Requester(requester_state) => match requester_state {
                RequesterState::RequestSent(request_sent_state) => request_sent_state.our_verkey(),
                RequesterState::Completed(completed_state) => completed_state.our_verkey(),
            },
            GenericDidExchange::Responder(responder_state) => match responder_state {
                ResponderState::ResponseSent(response_sent_state) => response_sent_state.our_verkey(),
                ResponderState::Completed(completed_state) => completed_state.our_verkey(),
            },
        }
    }

    pub fn their_did_doc(&self) -> &DidDocumentSov {
        match self {
            GenericDidExchange::Requester(requester_state) => match requester_state {
                RequesterState::RequestSent(request_sent_state) => request_sent_state.their_did_doc(),
                RequesterState::Completed(completed_state) => completed_state.their_did_doc(),
            },
            GenericDidExchange::Responder(responder_state) => match responder_state {
                ResponderState::ResponseSent(response_sent_state) => response_sent_state.their_did_doc(),
                ResponderState::Completed(completed_state) => completed_state.their_did_doc(),
            },
        }
    }

    pub async fn construct_request(
        ledger: Arc<dyn IndyLedgerRead>,
        config: ConstructRequestConfig,
    ) -> Result<(Self, Request), AriesVcxError> {
        let TransitionResult { state, output } =
            DidExchangeServiceRequester::<RequestSent>::construct_request(ledger, config).await?;
        Ok((
            GenericDidExchange::Requester(RequesterState::RequestSent(state)),
            output,
        ))
    }

    pub async fn handle_request(config: ReceiveRequestConfig) -> Result<(Self, Response), AriesVcxError> {
        let TransitionResult { state, output } =
            DidExchangeServiceResponder::<ResponseSent>::receive_request(config).await?;
        Ok((
            GenericDidExchange::Responder(ResponderState::ResponseSent(state)),
            output,
        ))
    }

    pub async fn handle_response(self, response: Response) -> Result<(Self, Complete), (Self, AriesVcxError)> {
        match self {
            GenericDidExchange::Requester(requester_state) => match requester_state {
                RequesterState::RequestSent(request_sent_state) => {
                    match request_sent_state.receive_response(response).await {
                        Ok(TransitionResult { state, output }) => {
                            Ok((GenericDidExchange::Requester(RequesterState::Completed(state)), output))
                        }
                        Err(TransitionError { state, error }) => {
                            Err((GenericDidExchange::Requester(RequesterState::RequestSent(state)), error))
                        }
                    }
                }
                RequesterState::Completed(_) => todo!("fail"),
            },
            GenericDidExchange::Responder(_) => todo!("fail"),
        }
    }

    pub fn handle_complete(self, complete: Complete) -> Result<Self, (Self, AriesVcxError)> {
        match self {
            GenericDidExchange::Requester(_) => todo!("fail"),
            GenericDidExchange::Responder(responder_state) => match responder_state {
                ResponderState::ResponseSent(response_sent_state) => {
                    match response_sent_state.receive_complete(complete) {
                        Ok(state) => Ok(GenericDidExchange::Responder(ResponderState::Completed(state))),
                        Err(TransitionError { state, error }) => Err((
                            GenericDidExchange::Responder(ResponderState::ResponseSent(state)),
                            error,
                        )),
                    }
                }
                ResponderState::Completed(_) => todo!("fail"),
            },
        }
    }
}
