use state_machine;
use game::GameProcessor;

#[derive(Debug, PartialEq, Eq, Primitive)]
pub enum EController {
    ControllerOne = 1,
    ControllerTwo = 2,
}

pub const PLAYER_ONE: EController = EController::ControllerOne;
pub const PLAYER_TWO: EController = EController::ControllerTwo;

pub fn end_turn(
    machine: GameProcessor<state_machine::core_states::Waiting>,
    _variables: (EController,),
) -> GameProcessor<state_machine::core_states::Waiting> {
    // 'machine' is the container of the processing state!
    
    return machine;
}

#[macro_export]
macro_rules! EndTurn {
    ($player:ident) => {
        |machine| {
            $crate::operations::play_options::end_turn(machine, ($player,))
        }
    }
}
