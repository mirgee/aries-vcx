#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestSent {
    pub invitation_id: String,
    pub request_id: String,
}
