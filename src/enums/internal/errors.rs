use enums::contracted::{EntityCreationError, EntityDataCreationError};


#[derive(Debug)]
pub enum GameConfigError {
    NoFormat,
    StartingPlayerOOB, // Out of Bounds!
    NoName(u32),
    NoHeroClass(u32),
    NoDeck(u32),
}

#[derive(Debug)]
pub enum GameCreationError {
    InvalidConfig(GameConfigError),
    InvalidEntityData(EntityDataCreationError),
    InvalidControllerData(EntityCreationError),
    HeroConstructionError(EntityCreationError), // Also used for heropower
}
