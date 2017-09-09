use std::fmt::{Debug, Display};

use contracts::state_machine::IMethod;
use enums::contracted::EExecutionStates;

pub trait IProgram: Debug + Display {
    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates;

    /// Instantly execute the provided method within this
    /// program.
    fn fast_execute(
        &self,
        method: Box<IMethod>,
    ) -> EExecutionStates;
}
