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
