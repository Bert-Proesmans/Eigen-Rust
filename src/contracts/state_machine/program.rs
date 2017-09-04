use std::fmt::{Debug, Display};

use enums::EExecutionStates;
use contracts::state_machine::{SharedState, Method};

pub trait Program: Debug + Display {
    fn shared_state(&self) -> &SharedState;
    fn method_list(&self) -> &i32;

    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates;

    /// Instantly execute the provided method within this program.
    fn fast_execute(&self, &mut Method) -> EExecutionStates;
}
