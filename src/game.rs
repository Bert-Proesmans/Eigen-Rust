use std::boxed::Box;

use entities::entity::IEntity;
use entities::core_entities;

use state_machine::core_states;
use state_machine::shared_data::SharedData;

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
    S: core_states::StateMarker,
{
    pub(crate) state: S,
    pub(crate) entities: Vec<Box<IEntity>>,
    pub(crate) program_state: SharedData,
}

impl GameProcessor<core_states::AwaitingStart> {
    pub fn new() -> Result<Self, String> {

        let game_entity = try!(core_entities::Game::new());

        Ok(Self {
            state: core_states::AwaitingStart::new(),
            entities: vec![
                Box::new(game_entity),
            ],
            program_state: SharedData::new(),
        })
    }
}
