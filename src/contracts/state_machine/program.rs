use std::fmt::{Debug, Display};

use contracts::entities::entity::IEntity;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::shared_state::ISharedState;

use enums::{EExecutionStates, EGameSteps};

/// Represents the object delegating all operations during
/// the game
///
/// The program is in fact the emulator which holds the
/// state of the game
/// and is responsible for executing the dynamic effects
/// (which are represented
/// by the IMethod trait).
pub trait IProgram<'program>: Debug + Display {
    /// Returns all created entities for the attached game
    fn all_entities(&self) -> Box<Iterator<Item = &(IEntity + 'program)> + 'program>;

    /// Returns the mutable shared state of this program
    fn shared_state_mut(&mut self) -> &mut (ISharedState<'program> + 'program);

    /// Process the next queued method
    fn process_next(&mut self) -> EExecutionStates;

    /// Instantly execute the provided method within this
    /// program
    fn fast_execute(
        &mut self,
        method: Box<IMethod>,
    ) -> EExecutionStates;

    /// Push a method into the queue for later execution
    fn register_method(
        &mut self,
        method: Box<IMethod>,
    );

    /// Transition the current program phase into a next one
    fn transition_step(
        &mut self,
        state: EGameSteps,
    );
}
