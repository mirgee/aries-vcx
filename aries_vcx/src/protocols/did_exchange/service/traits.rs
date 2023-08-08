pub trait ThreadId {
    fn thread_id(&self) -> &str;
}

pub trait ParentThreadId {
    fn parent_thread_id(&self) -> &str;
}
