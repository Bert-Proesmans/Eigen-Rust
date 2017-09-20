use contracts::cards::errors::Error as CardError;
use contracts::entities::entity_castable::errors::Error as CastError;
use contracts::entities::errors::Error as EntityError;
use slog;

use game::errors::Error as GameError;
use game_manager::errors::Error as ManagerError;

pub trait ErrorLogging<T> {
    fn log_unwrap(
        self,
        logger: &slog::Logger,
    ) -> T;
}

register_result_type!(CardError);
register_result_type!(EntityError);
register_result_type!(CastError);

register_result_type!(ManagerError);
register_result_type!(GameError);
