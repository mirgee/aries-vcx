#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSent {
    pub request_id: String,
    pub invitation_id: String,
}
