use entities::core_entities::EntityId;

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
pub struct GameZone {
    id: EZones,
    owner: EntityId,
    residents: Vec<EntityId>,
    triggers: Vec<u32>
}

impl GameZone {}
