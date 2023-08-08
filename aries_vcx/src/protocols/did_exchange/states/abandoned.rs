use super::traits::ThreadId;

// TODO: Add parent thread
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Abandoned {
    pub reason: String,
    pub request_id: String,
}

impl ThreadId for Abandoned {
    fn thread_id(&self) -> &str {
        self.request_id.as_str()
    }
}
