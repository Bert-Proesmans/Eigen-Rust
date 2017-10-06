use std::fmt;

use contracts::entities::entity::IEntity;
use contracts::entities::playable::IPlayable;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::program::IProgram;

use enums::{EActivationRequirements, EActivationTargets};

/// Representing an effect which is defined on a card
///
/// The actual object influencing the game state can be
/// found
/// under `effect_code`
pub trait ICardEffect<'effect>
    : fmt::Debug + fmt::Display + Sync {
    fn activation_target(&self) -> EActivationTargets;
    fn activation_requirement(&self) -> EActivationRequirements;
    fn remove_after_activation(&self) -> bool;

    fn effect_code(&self) -> &Option<Box<IMethod + 'effect>>;

    fn activate<'g, 'e: 'g>(
        &mut self,
        state: &'g mut IProgram,
        controller: &'e mut IEntity,
        source: &'e mut IPlayable,
        target: Option<&'e mut IPlayable<'e>>,
    );
}
