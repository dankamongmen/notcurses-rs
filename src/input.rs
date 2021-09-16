#![allow(dead_code)]

use crate::sys::{self, NcInput};

/// Reads and decodes input events.
///
/// Reads from stdin and decodes the input to stdout,
/// including synthesized events and mouse events.
#[derive(Copy, Clone, Debug)]
pub struct Input {
    pub(crate) ncinput: NcInput,
}

impl Input {
    pub fn new_empty() -> Self {
        Self {
            ncinput: NcInput::new_empty(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl From<Input> for NcInput {
    fn from(input: Input) -> Self {
        input.ncinput
    }
}

impl From<NcInput> for Input {
    fn from(ni: NcInput) -> Self {
        Self { ncinput: ni }
    }
}
