use std::fmt;

use contracts::cards::card::ICard;
use contracts::entities::entity::IEntity;

use super::errors::*;

/// Trait supporting easy initialisation of structures
/// implementing IEntity
pub trait IEntityInitializable: fmt::Debug + fmt::Display {
    /// Constructs a new IEntity object from the provided
    /// data
    ///
    /// The new object is allocated on the heap and boxed
    /// to automatically handle
    /// upcasting.
    ///
    /// # Panics
    ///
    /// Panics when the EntityData object couldn't be
    /// constructed during
    /// construction.
    ///
    fn new(
        entity_id: u32,
        &'static ICard,
    ) -> Result<Box<IEntity>>;
}
