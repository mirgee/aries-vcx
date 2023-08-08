// TODO: What should this state contain besides the cause of failure?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Abandoned {
    pub reason: String,
}
