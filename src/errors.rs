use error_chain::ChainedError;
use slog;

use contracts::cards::errors::Error as CardError;
// use contracts::entities::entity_castable::errors::Error
// as CastError;
// use contracts::entities::errors::Error as EntityError;


// use game::errors::Error as GameError;
// use game_manager::errors::Error as ManagerError;

pub trait ResultLogging<T> {
    /// Prints the error chain text to the provided logger.
    ///
    /// # Panics
    /// Panics when the value is equivalent to an error.
    fn log_unwrap(
        self,
        logger: &slog::Logger,
    ) -> T;

    /// Prints the error chain text to the output stream.
    ///
    /// # Panics
    /// Panics when the value is equivalent to an error.
    fn print_unwrap(self) -> T;
}

register_result_type!(CardError);
// register_result_type!(EntityError);
// register_result_type!(CastError);

// register_result_type!(ManagerError);
// register_result_type!(GameError);
