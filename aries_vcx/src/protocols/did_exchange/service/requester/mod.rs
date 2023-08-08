mod request_sent;

use super::{attach_to_ddo_sov, create_our_did_document, DidExchangeService};

pub use request_sent::config::{ConstructRequestConfig, PairwiseConstructRequestConfig, PublicConstructRequestConfig};

#[derive(Clone, Copy, Debug)]
pub struct Requester;

pub type DidExchangeServiceRequester<S> = DidExchangeService<Requester, S>;
