// #![deny(missing_docs,
// missing_debug_implementations,
// missing_copy_implementations,
// trivial_casts, trivial_numeric_casts,
// unsafe_code,
// unstable_features,
// unused_import_braces, unused_qualifications)]

#![allow(dead_code, unused_imports)]


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
#[macro_use]
pub extern crate slog;
extern crate slog_stdlog;
extern crate chrono;

#[macro_use]
mod macros;
pub mod enums;
mod errors;

mod contracts;
mod cards;
mod card_sets;
mod game_manager;
mod game;
pub mod state_machine;
pub mod operations;

// mod effects;

pub mod prelude {
    // This module will re-export all important types and
    // traits.

    pub use errors::*;

    // All contracts are put in the prelude to be able to use
    // their defined methods everywhere.

    pub use contracts::*;

    pub use cards::card_container::CARDS;

    pub use game::config::GameConfig;
    pub use game_manager::GameManager;

    // Declarations and constants

    use enums::EControllers;
    pub const PLAYER_ONE: EControllers = EControllers::ControllerOne;
    pub const PLAYER_TWO: EControllers = EControllers::ControllerTwo;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
