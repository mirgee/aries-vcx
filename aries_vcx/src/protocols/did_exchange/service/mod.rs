pub mod generic;
pub mod requester;
pub mod responder;

use did_doc_sov::DidDocumentSov;

use crate::protocols::mediated_connection::pairwise_info::PairwiseInfo;

use super::protocol::DidExchangeProtocol;

// Should be able to:
// * accept OOB invitation and create didexchange protocol SM, request and hold our pwdid
// * accept request and create didexchange protocol SM, response, and hold DDO of counterparty and our pwdid
// * accept response and transition the existing SM to complete, complete message, and hold the final DDO of counterparty and our pwdid
// The final result should be something which would be eventually uniform for both didexchange and
// connection protocols. I.e. this "service" can be didexchange-specific, but the final outcome
// should be a "ConnectionRecord", which would hold all the information necessary to send messages
// to the counterparty.
// 1.) Should this be two separate structs (requester, responder), or should it
// be a single struct similar to the protocol?
// * What would be shared: thread id and state getters, and the final outcome (ConnectionRecord)
// * The interface will be similar to that of ConnectionRecord in the intermediate states (bootstrap
// vs. final DDO)
// 2.) Should it be generic over the state?
#[derive(Debug, Clone, PartialEq)]
pub struct DidExchangeService<I, S> {
    sm: DidExchangeProtocol<I, S>,
    pairwise_info: PairwiseInfo,
    did_document: DidDocumentSov,
}

impl<I, S> DidExchangeService<I, S> {
    pub fn our_verkey(&self) -> &str {
        &self.pairwise_info.pw_vk
    }

    pub fn their_did_doc(&self) -> &DidDocumentSov {
        &self.did_document
    }
}

impl<I, S> DidExchangeService<I, S> {
    pub fn from_parts(
        sm: DidExchangeProtocol<I, S>,
        did_document: DidDocumentSov,
        pairwise_info: PairwiseInfo,
    ) -> Self {
        Self {
            sm,
            pairwise_info,
            did_document,
        }
    }
}
