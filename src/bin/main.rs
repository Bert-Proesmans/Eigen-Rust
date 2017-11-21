#[macro_use]
extern crate eigen_rust;

use eigen_rust::game::{GameFactory, GameProcessor};
use eigen_rust::entities::core_entities::{PLAYER_ONE, PLAYER_TWO};
use eigen_rust::state_machine::core_states;
use eigen_rust::state_machine::trigger_states;
use eigen_rust::game_triggers;

fn main() {
    let game = GameFactory::new(0u32).unwrap();

    // Start the game.
    let game: GameProcessor<core_states::Trigger<trigger_states::StartGame>> = game.into();

    // Mulligan (Pre game)
    // ..Skip it..
    // Go into waiting for input state.
    let game: GameProcessor<core_states::Input> = game.into();


    // In game
    let task = EndTurn!(PLAYER_ONE);
    let game = (task)(game);
    if game.is_err() {
        panic!("Unexpected game end!");
    }

    let task = EndTurn!(PLAYER_TWO);
    let game = (task)(game.unwrap());
    if game.is_err() {
        panic!("Unexpected game end!");
    }

    // let task = Concede!(PLAYER_ONE);
    // let game = (task)(game);

    // // Validate the game state is FINISHED.
    // if game.is_err() {
    //     println!("Game completed succesfully!");
    // } else {
    //     panic!("Game didn't complete properly!")
    // }
}
