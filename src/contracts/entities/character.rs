use std::fmt;

use contracts::entities::entity::IEntity;
use contracts::entities::playable::IPlayable;

/// Represents an entity which behaves like an actor within
/// the game
///
/// A character can perform actions which affect other
/// characters. The most
/// important property is that a character can die!
pub trait ICharacter
    : fmt::Debug + fmt::Display + IEntity + IPlayable {
    // TODO; Implement
}
