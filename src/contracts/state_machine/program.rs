use std::fmt::{Debug, Display};

use contracts::state_machine::{Method, SharedState};
use enums::EExecutionStates;

pub trait Program: Debug + Display {
    fn shared_state(&self) -> &SharedState;
    fn method_list(&self) -> &i32;

    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates;

    /// Instantly execute the provided method within this
    /// program.
    fn fast_execute(&self, &mut Method) -> EExecutionStates;
}
