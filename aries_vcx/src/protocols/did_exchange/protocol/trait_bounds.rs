pub trait ThreadId {
    fn thread_id(&self) -> &str;
}

pub trait RequestId {
    fn request_id(&self) -> &str;
}

pub trait InvitationId {
    fn invitation_id(&self) -> &str;
}

pub trait HandleProblem {}
