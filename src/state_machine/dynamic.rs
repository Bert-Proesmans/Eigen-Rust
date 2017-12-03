use std::convert;

use game::GameMachine;
use state_machine::{core_states, wait_states};
use state_machine::action_trigger_states as ats;

use entities::core_entities::EntityId;
use game_tags::EGameTags;

#[derive(Debug)]
pub enum DynamicOperations {
    DecreaseOne,
    IncreaseOne,
    Increase(u32),
    Decrease(u32),
}

impl<'mx, T> GameMachine<'mx, core_states::Death<T>>
where
    T: core_states::StateTriggerable + Default,
{
    pub fn death_processing(self) -> Result<Self, String> {
        unimplemented!()
    }
}

impl<'mx, T /*, Y */> GameMachine<'mx, core_states::Effect<T>>
where
	T: core_states::StateTriggerable + Default,
    // Y: core_states::StateTriggerable + Default,
	// GameMachine<'mx, core_states::Trigger<Y>>: convert::From<GameMachine<'mx, core_states::Effect<T>>>,
    GameMachine<'mx, core_states::Death<T>>: convert::From<GameMachine<'mx, core_states::Effect<T>>>,
{
    pub fn signal(self, entity: EntityId, tag: EGameTags, operation: DynamicOperations) -> Self {
        // Store effect operations.
        unimplemented!()
    }

    pub fn update(self) -> Result<GameMachine<'mx, core_states::Death<T>>, String> {
        // Execute all stored operations.
        // Triggers matching the executed effect are run depth-first.
        // TODO; Invalidation check for operations under certain circumstances.
        //      -> Tag values can never be < 0 etc ..
        unimplemented!()
    }
}
