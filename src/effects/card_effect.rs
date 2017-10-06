use std::fmt;

use contracts::effects::card_effect::ICardEffect;
use contracts::entities::entity::IEntity;
use contracts::entities::playable::IPlayable;
use contracts::state_machine::program::IProgram;
use contracts::state_machine::method::IMethod;

use state_machine::shared_state::{REG_EID_ONE, REG_EID_TWO, REG_EID_THREE};

pub const EID_SOURCE: u32 = REG_EID_ONE;
pub const EID_TARGET: u32 = REG_EID_TWO;
pub const EID_CONTROLLER: u32 = REG_EID_THREE;

use enums::{EActivationTargets, EActivationRequirements};

#[derive(Debug, Default)]
pub struct CardEffect {
    activation_target: EActivationTargets,
    activation_requirement: EActivationRequirements,
    remove_after_activation: bool,
    effect: Option<Box<IMethod>>
}

impl fmt::Display for CardEffect {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "EFFECT [TODO]")
    }
}

unsafe impl Sync for CardEffect {}

impl ICardEffect for CardEffect {
    fn activation_target(&self) -> EActivationTargets {
        self.activation_target
    }

    fn activation_requirement(&self) -> EActivationRequirements {
        self.activation_requirement
    }

    fn remove_after_activation(&self) -> bool {
        self.remove_after_activation
    }

    fn effect_code<'e>(&'e self) -> &'e Option<Box<IMethod + 'static>> {
        &self.effect
    }

    fn activate<'e, 's, 'p :'s>(&'e mut self, state: &'s mut IProgram,
        controller: &'p mut IEntity, source: &'p mut IPlayable, target: Option<&'p mut IPlayable>) {

        if self.effect.is_none() {
            return;
        }

        let shared_state = state.shared_state_mut();
        // Store ID's in the shared state.
        shared_state.set_register(EID_CONTROLLER, controller.id() as i32);
        shared_state.set_register(EID_SOURCE, source.id() as i32);
        if let Some(target_entity) = target {
            shared_state.set_register(EID_TARGET, target_entity.id() as i32);
        } else {
            shared_state.set_register(EID_TARGET, 0); // No target
        }

        // Run method.
        self.effect.as_ref().unwrap().run(state);
    }
}
