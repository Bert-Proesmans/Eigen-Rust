
#[derive(Debug, PartialEq, Eq, Hash, Primitive)]
pub enum EGameTags {
    // Core tags
    EntityId = 1,
    TurnPlayerIdx = 2,

    RemainingTurns = 10,

    MarkerEndCore = 500 // Internal tags
}
