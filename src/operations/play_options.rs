use game_manager::GameManager;

use enums::{EControllers, EExecutionStates};

pub fn end_turn_operation(for_controller: EControllers) -> EExecutionStates {
    EExecutionStates::Invalid
}

#[macro_export]
macro_rules! EndTurn {
    ($controller:expr) => { $crate::state_machine::method::MethodOne::construct($crate::operations::play_options::end_turn_operation, $controller).unwrap(); }
}
