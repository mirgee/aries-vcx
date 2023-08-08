#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSent {
    pub invitation_id: String,
    pub request_id: String,
}
