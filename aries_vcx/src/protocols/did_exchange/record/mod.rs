use did_doc_sov::DidDocumentSov;

use crate::protocols::mediated_connection::pairwise_info::PairwiseInfo;

// TODO: Perhaps we should store our and their DDO?
// But that's really not necessary, we just need to be able to decrypt a message on arrival using
// our verkey
// I think this should be a struct, and not a trait. It is primarily a data container, not a set of
// behaviors.
// TODO: Consider that one day we will want to store did documents with other than sovrin extra
// fields
pub struct ConnectionRecord {
    did_document: DidDocumentSov,
    pairwise_info: PairwiseInfo,
}

impl ConnectionRecord {
    pub fn from_parts(did_document: DidDocumentSov, pairwise_info: PairwiseInfo) -> Self {
        Self {
            did_document,
            pairwise_info,
        }
    }

    pub fn did_document(&self) -> &DidDocumentSov {
        &self.did_document
    }

    pub fn pairwise_info(&self) -> &PairwiseInfo {
        &self.pairwise_info
    }
}
