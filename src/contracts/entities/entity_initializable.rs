use std::fmt;

use contracts::cards::card::ICard;
use contracts::entities::entity::IEntity;

use super::errors::*;

pub trait IEntityInitializable: fmt::Debug + fmt::Display {
    fn new(
        entity_id: u32,
        &'static ICard,
    ) -> Result<Box<IEntity>>;
}
