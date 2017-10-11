pub mod card;
pub mod game;
pub mod evaluation;
pub mod operations;

pub use self::card::*;
pub use self::evaluation::*;
pub use self::game::*;
pub use self::operations::*;

impl Default for ECardTypes {
    fn default() -> Self {
        ECardTypes::Invalid
    }
}

impl Default for ECardSets {
    fn default() -> Self {
        ECardSets::NoSet
    }
}

impl Default for EActivationTargets {
    fn default() -> Self {
        EActivationTargets::NoTarget
    }
}

impl Default for EActivationRequirements {
    fn default() -> Self {
        EActivationRequirements::NoReq
    }
}

impl Default for EExecutionStates {
    fn default() -> Self {
        EExecutionStates::Invalid
    }
}
