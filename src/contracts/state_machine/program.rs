use std::fmt::{Debug, Display};

use contracts::entities::entity::IEntity;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::shared_state::ISharedState;

use enums::{EExecutionStates, EGameSteps};

pub trait IProgram<'a>: Debug + Display {
    fn all_entities(&self) -> Box<Iterator<Item = &'a IEntity>>;

    fn shared_state_mut(&mut self) -> &'a mut ISharedState;

    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates;

    /// Instantly execute the provided method within this
    /// program.
    fn fast_execute(
        &mut self,
        method: Box<IMethod>,
    ) -> EExecutionStates;

    fn register_method(
        &mut self,
        method: Box<IMethod>,
    );

    fn transition_step(
        &mut self,
        state: EGameSteps,
    );
}
