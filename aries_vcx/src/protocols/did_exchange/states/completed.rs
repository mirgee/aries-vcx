use std::clone::Clone;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Completed {
    pub invitation_id: String,
    pub request_id: String,
}
