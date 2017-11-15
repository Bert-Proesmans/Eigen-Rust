#[macro_use]
extern crate eigen_rust;

use eigen_rust::game::{GameFactory,GameProcessor};
use eigen_rust::state_machine::core_states;
use eigen_rust::operations::play_options::PLAYER_ONE;

fn main() {
   let game = GameFactory::new(0u32).unwrap();

   // Mulligan (Pre game)
   // ..Skip it..
   let game: GameProcessor<core_states::Waiting> = game.into();


   // In game
   let task = EndTurn!(PLAYER_ONE);
   let game = (task)(game);

   // let task = Concede!(PLAYER_ONE);
   // let game = (task)(game);

   // Validate the game state is FINISHED.
   
}
