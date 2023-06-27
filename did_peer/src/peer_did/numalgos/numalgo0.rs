use super::traits::Numalgo;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Numalgo0;

impl Numalgo for Numalgo0 {
    const NUMALGO_CHAR: char = '0';

    fn instance() -> Self {
        Self
    }
}
