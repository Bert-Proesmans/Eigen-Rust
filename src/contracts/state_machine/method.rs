use std::fmt::{Debug, Display};

use contracts::state_machine::program::IProgram;

use enums::EExecutionStates;

/// Represents an effect which is executed during the game
///
/// Comparable to anonymous functions. These instances can
/// be invoked by calling `run(..)` and will execute
/// the embedded code.
///
/// Methods don't take arguments to run and work entirely
/// based on information within the shared state,
/// which is held by the program instance.
pub trait IMethod: Debug + Display + Sync {
    /// Returns the state value of this method object
    fn state(&self) -> EExecutionStates;

    /// Run the code held by this method object
    fn run(
        &mut self,
        state: &mut IProgram,
    ) -> EExecutionStates;
}
