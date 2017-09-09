use std::fmt;

use contracts::state_machine::{IMethod, IProgram};

use core::state_machine::SharedState;

use enums::contracted::EExecutionStates;

#[derive(Debug)]
pub struct Program<'program> {
    state: SharedState<'program>,
    method_queue: Vec<Box<IMethod>>
}

impl<'program> fmt::Display for Program<'program> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "PROGRAM [TODO]")
    }
}

impl<'program> Program<'program> {
    pub fn new() -> Self {
        Self {
            state: SharedState::new(),
            method_queue: vec![]
        }
    }
}

impl<'program> IProgram for Program<'program> {
    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates {
        EExecutionStates::Invalid
    }

    /// Instantly execute the provided method within this
    /// program.
    fn fast_execute(
        &self,
        method: Box<IMethod>,
    ) -> EExecutionStates {
        EExecutionStates::Invalid
    }
}
