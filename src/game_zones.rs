use std::collections::HashMap;

use entities::core_entities::{EController, EntityId};
use game_triggers::StoredTrigger;

#[derive(Debug, PartialEq, Eq, Hash, Primitive)]
pub enum EZones {
    Invalid = 0,
    Play = 1,
    Deck = 2,
    Hand = 3,
    Graveyard = 4,
    RemovedFromGame = 5,
    Setaside = 6,
    Secret = 7
}

#[derive(Debug)]
pub struct ZoneContainer {
    zones: HashMap<(EController, EZones), GameZone>
}

impl ZoneContainer {
    pub fn new() -> Self {
        Self { zones: hashmap!{} }
    }
}

#[derive(Debug)]
pub struct GameZone {
    id: EZones,
    owner: EntityId,
    residents: Vec<EntityId>,
    triggers: Vec<StoredTrigger>
}

impl GameZone {}
