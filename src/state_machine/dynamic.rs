use std::convert;

use game::GameMachine;
use state_machine::{action_states, core_states, trigger_states, wait_states};

use entities::core_entities::EntityId;
use game_tags::EGameTags;

impl<T> GameMachine<core_states::Death<T>>
where
    T: core_states::StateTriggerable + Default,
{
    pub fn death_processing(self) -> Result<Self, String> {
        unimplemented!()
    }
}

impl<T> GameMachine<core_states::Trigger<T>>
where
	T: core_states::StateTriggerable + Default,
	GameMachine<core_states::Effect<T>>: convert::From<GameMachine<core_states::Trigger<T>>>,
{
    pub fn signal(self, entity: EntityId, tag: EGameTags, old: u32, new: u32)
    	-> Result<GameMachine<core_states::Death<T>>, String>
    {
    	let machine = self;

		// Collect active triggers.

		// Validate collected triggers against current change.

		// Transition to effect state.
		let machine: GameMachine<core_states::Effect<T>> = machine.into();

		// Execute each trigger.

		// Move into death processing state.
		// Caller decides if death processing is done or not!
    	Ok(machine.into())
    }
}
