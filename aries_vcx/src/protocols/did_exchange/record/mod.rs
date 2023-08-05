use did_doc_sov::DidDocumentSov;
use public_key::Key;

// TODO: Perhaps we should store our and their DDO?
// But that's really not necessary, we just need to be able to decrypt a message on arrival using
// our verkey
// I think this should be a struct, and not a trait. It is primarily a data container, not a set of
// behaviors.
// TODO: Consider that one day we will want to store did documents with other than sovrin extra
// fields
pub struct ConnectionRecord {
    their_did_document: DidDocumentSov,
    our_verkey: Key,
}

impl ConnectionRecord {
    pub fn from_parts(did_document: DidDocumentSov, our_verkey: Key) -> Self {
        Self {
            their_did_document: did_document,
            our_verkey,
        }
    }

    pub fn did_document(&self) -> &DidDocumentSov {
        &self.their_did_document
    }
}
