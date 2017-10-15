use std::fmt;
use std::marker::PhantomData;

use contracts::state_machine::method::IMethod;
use contracts::state_machine::program::IProgram;

use enums::EExecutionStates;

pub struct Method<F>
where
    F: Fn(&mut IProgram) -> EExecutionStates,
{
    last_state: EExecutionStates,
    delegate: F
}

impl<F> fmt::Debug for Method<F>
where
    F: Fn(&mut IProgram) -> EExecutionStates,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "[TODO]")
    }
}

impl<F> fmt::Display for Method<F>
where
    F: Fn(&mut IProgram) -> EExecutionStates,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "[TODO]")
    }
}

impl<F> IMethod for Method<F>
where
    F: Fn(&mut IProgram) -> EExecutionStates,
{
    fn state(&self) -> EExecutionStates {
        self.last_state
    }

    fn run(
        &mut self,
        state: &mut IProgram,
    ) -> EExecutionStates {
        // TODO; Only run when in specific states.
        self.last_state = (self.delegate)(state);
        self.last_state
    }
}

impl<F> Method<F>
where
    F: Fn(&mut IProgram) -> EExecutionStates,
{
    pub fn construct(delegate: F) -> Self {
        Self {
            last_state: EExecutionStates::default(),
            delegate: delegate
        }
    }
}
