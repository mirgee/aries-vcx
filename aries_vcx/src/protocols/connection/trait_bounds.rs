use diddoc_legacy::aries::diddoc::AriesDidDoc;
use messages::msg_fields::protocols::discover_features::{disclose::Disclose, ProtocolDescriptor};

pub trait TheirDidDoc {
    fn their_did_doc(&self) -> &AriesDidDoc;
}

pub trait BootstrapDidDoc: TheirDidDoc {
    fn bootstrap_did_doc(&self) -> &AriesDidDoc {
        self.their_did_doc()
    }
}

pub trait ThreadId {
    fn thread_id(&self) -> &str;
}

pub trait CompletedState {
    fn remote_protocols(&self) -> Option<&[ProtocolDescriptor]>;

    fn handle_disclose(&mut self, disclose: Disclose);
}

pub trait HandleProblem {}
