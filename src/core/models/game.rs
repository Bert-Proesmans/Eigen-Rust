use core::models::entities::{Entity, EntityData};
use core::GameConfig;
use core::models::gameconfig::MAX_PLAYERS;

const GAME_ENTITY_ID: u32 = 1;

#[derive(Debug)]
pub struct Game {
    data: EntityData,
    config: Option<GameConfig>,

    players: [i32; MAX_PLAYERS], // TODO
    entities: Vec<Box<Entity>>, // All entities except the game itself

    next_eid: u32,
    next_oop: u32,
}

impl Game {
    // pub fn new(config: GameConfig) -> Self {}
}
