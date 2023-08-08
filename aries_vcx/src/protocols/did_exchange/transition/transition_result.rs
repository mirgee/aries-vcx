// TODO: Somehow enforce using both
#[must_use]
pub struct TransitionResult<T, U> {
    pub state: T,
    pub output: U,
}

impl<T, U> From<(T, U)> for TransitionResult<T, U> {
    fn from(value: (T, U)) -> Self {
        TransitionResult {
            state: value.0,
            output: value.1,
        }
    }
}
