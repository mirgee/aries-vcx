mod response_sent;

pub use response_sent::config::ReceiveRequestConfig;

use super::DidExchangeService;

#[derive(Clone, Copy, Debug)]
pub struct Responder;

pub type DidExchangeServiceResponder<S> = DidExchangeService<Responder, S>;
