// DBG
#![allow(unused_variables, unused_imports)]

#[macro_use]
extern crate eigen_rust;

use eigen_rust::entities::core_entities::{PLAYER_ONE, PLAYER_TWO};
use eigen_rust::game::GameFactory;

fn main() {
    let game = GameFactory::new(0u32).expect("Couldn't build a new game");

    // Start the game.
    // Mulligan (Pre game) ..skip it..
    let start_game_task = StartGame!();
    let game = (start_game_task)(game);
    if game.is_err() {
        panic!("Unexpected game end!");
    }

    // In game
    let task_endturn_player_one = EndTurn!(PLAYER_ONE);
    let task_endturn_player_two = EndTurn!(PLAYER_TWO);

    let game = (task_endturn_player_one)(game.unwrap());
    if game.is_err() {
        panic!("Unexpected game end!");
    }

    let game = (task_endturn_player_two)(game.unwrap());
    if game.is_err() {
        panic!("Unexpected game end!");
    }

    // let task_concede_player_one = Concede!(PLAYER_ONE);
    // let game = (task_concede_player_one)(game);

    // // Validate the game state is FINISHED.
    // if game.is_err() {
    //     println!("Game completed succesfully!");
    // } else {
    //     panic!("Game didn't complete properly!")
    // }
}
