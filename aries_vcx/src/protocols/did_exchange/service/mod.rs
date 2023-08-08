mod helpers;
mod traits;

pub mod generic;
pub mod requester;
pub mod responder;

pub use helpers::generate_keypair;
use messages::{
    decorators::thread::Thread,
    msg_fields::protocols::did_exchange::problem_report::{
        ProblemCode, ProblemReport, ProblemReportContent, ProblemReportDecorators,
    },
};
use uuid::Uuid;

use std::marker::PhantomData;

use did_doc_sov::DidDocumentSov;
use public_key::Key;

use self::traits::ThreadId;

use super::{states::abandoned::Abandoned, transition::transition_result::TransitionResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DidExchangeService<I, S> {
    state: S,
    initiation_type: PhantomData<I>,
    // TODO: Do we NEED to store our DDO or does it suffice to store the verkey?
    our_verkey: Key,
    their_did_document: DidDocumentSov,
}

// TODO: Enable SOME states to transition to Abandoned state
// TODO: Add "thin state" getter
impl<I, S> DidExchangeService<I, S> {
    pub fn our_verkey(&self) -> &Key {
        &self.our_verkey
    }

    pub fn their_did_doc(&self) -> &DidDocumentSov {
        &self.their_did_document
    }

    pub fn fail(
        self,
        reason: String,
        problem_code: Option<ProblemCode>,
    ) -> TransitionResult<DidExchangeService<I, Abandoned>, ProblemReport>
    where
        Self: ThreadId,
    {
        let problem_report = {
            let id = Uuid::new_v4().to_string();
            let content = ProblemReportContent {
                problem_code,
                explain: Some(reason.clone()),
            };
            let decorators = ProblemReportDecorators {
                // TODO: Set thid of the conversation
                thread: Thread::new(self.thread_id().to_string()),
                // TODO: Building a message is pain
                localization: None,
                timing: None,
            };
            ProblemReport::with_decorators(id, content, decorators)
        };
        TransitionResult {
            state: DidExchangeService {
                state: Abandoned { reason },
                initiation_type: PhantomData,
                our_verkey: self.our_verkey,
                their_did_document: self.their_did_document,
            },
            output: problem_report,
        }
    }
}

impl<I, S> DidExchangeService<I, S> {
    pub fn from_parts(state: S, their_did_document: DidDocumentSov, our_verkey: Key) -> Self {
        Self {
            state,
            initiation_type: PhantomData,
            our_verkey,
            their_did_document,
        }
    }
}
