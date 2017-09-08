use std::fmt::{Debug, Display};

use enums::EExecutionStates;

/// Comparable to anonymous functions. These instances can
/// be invoked
/// by calling `run(..)` and will execute the embedded code.
///
/// Methods don't take arguments to run and work entirely
/// based on
/// information within the shared state, which is held by
/// the game
/// instance.
pub trait Method: Debug + Display {
    /// Gets the state value of this instance.
    fn state(&self) -> EExecutionStates;
    /// Run the embedded code of this method.
    fn run(&self) -> EExecutionStates;
}
