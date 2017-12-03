use num_traits::{FromPrimitive, ToPrimitive};

use state_machine::core_states::{StateCore, Wait};
use state_machine::wait_states::Start;

use entities::core_entities::{EController, EntityId, GAME_ENTITY_ID};
use entities::entity::IEntity;
use game_tags::EGameTags;

#[derive(Debug)]
pub struct GameFactory {}

impl GameFactory {
    pub fn new<'fx>(_cfg: u32) -> Result<GameMachine<'fx, Wait<Start>>, String> {
        let wait_state = Wait::<Start>::default();

        // TODO; Game entity setup

        // TODO; Essential triggers setup

        Ok(GameMachine {
            state: wait_state,
            entities: vec![]
        })
    }
}

#[derive(Debug)]
pub struct GameMachine<'machine, S>
where
    S: StateCore,
{
    pub(crate) state: S,
    pub(crate) entities: Vec<Box<IEntity<'machine>>>
}

impl<'mx, S> GameMachine<'mx, S>
where
    S: StateCore,
{
    pub fn get_current_player_eid(&self) -> EntityId {
        let current_controller_idx = self.get_entity(GAME_ENTITY_ID).unwrap().tag_value(EGameTags::TurnPlayerIdx);
        // Validation step.
        EController::from_u32(current_controller_idx).expect("Invalid controller index found!");
        // Calculate result
        (GAME_ENTITY_ID + current_controller_idx)
    }

    pub fn get_entity(
        &self,
        entity_id: u32,
    ) -> Option<&Box<IEntity<'mx>>> {
        // Bounds check is automatically done.
        // EntityID's are 1-indexed!
        self.entities.get((entity_id - 1) as usize)
    }

    pub fn get_entity_mut(
        &mut self,
        entity_id: u32,
    ) -> Option<&mut Box<IEntity<'mx>>> {
        // Bounds check is automatically done.
        // EntityID's are 1-indexed!
        self.entities.get_mut((entity_id - 1) as usize)
    }
}
