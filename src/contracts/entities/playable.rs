use std::fmt;

use contracts::entities::entity::IEntity;

/// Represents an entity which can be "played"
///
/// To explain "Played" you also need to know about the
/// Zone concept.
/// While the game is running entities are stored within
/// zones.
/// One of the zones is the HAND zone, which can only hold
/// IPlayable entities.
/// When the player plans to activate the effects of a
/// specific card, he "plays"
/// the entity constructed from that card.
/// As a result of this action that entity moves from HAND
/// zone to PLAY zone,
/// where it's effects are executed.
pub trait IPlayable: fmt::Debug + fmt::Display + IEntity {
    // TODO; Implement
}
