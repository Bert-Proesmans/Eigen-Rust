use contracts::state_machine::program::IProgram;
use enums::{EControllers, EExecutionStates};

macro_rules! construct_operation {
    ($dol:tt $op_name:ident<$name:ident>; $($arg_name:ident = $arg_type:ty),*; $body:tt) => {
        #[macro_export]
        macro_rules! $name {
            ( $($dol $arg_name:expr),* ) => {
                Box::new($crate::state_machine::method::Method::construct(
                    |s| $crate::operations::play_options::$op_name(s, $($dol $arg_name),*)
                ));
            };
        }

        pub fn $op_name(state: &mut $crate::contracts::state_machine::program::IProgram,
                        $($arg_name: $arg_type),*) -> $crate::enums::EExecutionStates
        {
            $body
        }
    };
}

// Note: All operations MUST return an invariant of type
// EExecutionStates!
// Note: arguments MUST be copy types!
// Note: variable `state`, which is a mutable IProgram is
// automatically inserted into
// the operation scope.

// End Turn task.
construct_operation!($ end_turn_operation<EndTurn>; _for_controller = EControllers;
    {
        EExecutionStates::Invalid
    }
);
