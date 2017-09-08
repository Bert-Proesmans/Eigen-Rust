pub mod contracted;
pub mod internal;
mod card;
mod game;

//////////////////////////////////////////////////////////////
// The default underlying type for all enumerations is u32!
// //
//////////////////////////////////////////////////////////////

// Re-export all enums from child modules through this
// module.

pub use self::card::*;
pub use self::game::*;
pub use self::internal::*;

//////////////////////////////////////////////
// Defaults for enums (only when necessary) //
//////////////////////////////////////////////

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
