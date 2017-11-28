use std::boxed::Box;

use entities::core_entities;
use entities::entity::IEntity;
use game_triggers;
use state_machine::{core_states, trigger_states, dynamic};
use state_machine::shared_data::SharedData;
use game_zones;

// Shortcut
pub type GameTriggerState<T> = GameProcessor<core_states::Trigger<T>>;

#[derive(Debug)]
pub struct GameFactory {}

impl GameFactory {
    pub fn new(_config: u32) -> Result<GameProcessor<core_states::AwaitingStart>, String> {
        GameProcessor::new()
    }
}

#[derive(Debug)]
pub struct GameProcessor<S>
where
    S: core_states::CoreState,
{
    pub(crate) state: S,
    pub(crate) entities: Vec<Box<IEntity>>,
    pub(crate) zones: game_zones::ZoneContainer,
    pub(crate) program_state: SharedData,
    pub(crate) triggers: game_triggers::TriggerContainer
}

impl<S> GameProcessor<S>
where
    S: core_states::CoreState,
{
    pub fn get_entity(
        &self,
        entity_id: u32,
    ) -> Option<&Box<IEntity>> {
        // Bounds check is automatically done.
        // EntityID's are 1-indexed!
        self.entities.get((entity_id - 1) as usize)
    }

    pub fn get_entity_mut(
        &mut self,
        entity_id: u32,
    ) -> Option<&mut Box<IEntity>> {
        // Bounds check is automatically done.
        // EntityID's are 1-indexed!
        self.entities.get_mut((entity_id - 1) as usize)
    }
}

impl GameProcessor<core_states::AwaitingStart> {
    pub fn new() -> Result<Self, String> {

        let game_entity = try!(core_entities::Game::new());
        let current_obj = Self {
            state: core_states::AwaitingStart::new(),
            entities: vec![Box::new(game_entity)],
            zones: game_zones::ZoneContainer::new(),
            program_state: SharedData::new(),
            triggers: game_triggers::TriggerContainer::new()
        };

        let current_obj = try!(current_obj._build_default_triggers());

        Ok(current_obj)
    }

    fn _setup_game(self) -> Result<Self, String> {
        // TODO
        Ok(self)
    }

    fn _build_controllers(self) -> Result<Self, String> {
        // TODO
        Ok(self)
    }

    fn _build_zones(self) -> Result<Self, String> {
        // TODO
        Ok(self)
    }

    fn _build_default_triggers(mut self) -> Result<Self, String> {   
        let end_turn_trigger = game_triggers::TriggerBuilder::default()
                            .effect(dynamic::MethodTrigger::new(self::_test))
                            .build()
                            .unwrap();
        self.triggers.store_global_trigger(Box::new(end_turn_trigger));
        Ok(self)
    }
}

fn _test(_machine: GameTriggerState<trigger_states::EndTurn>) -> Result<GameTriggerState<trigger_states::EndTurn>, dynamic::EExecutionStates> {
        println!("EndOfTurn trigger");
        Ok(_machine)
    }
