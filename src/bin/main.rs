#[macro_use]
extern crate eigen_rust;


fn main() {
   let factory = eigen_rust::GameFactory::new(0u32);
   let game = factory.start_game();

   let task = EndTurn!(PLAYER_ONE);
   let game = (task)(game);

   let task = Concede!(PLAYER_ONE);
   factory.process(task);

   // Validate the game state is FINISHED.
}
