use super::traits::Numalgo;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Numalgo1;

impl Numalgo for Numalgo1 {
    const NUMALGO_CHAR: char = '1';

    fn instance() -> Self {
        Self
    }
}
