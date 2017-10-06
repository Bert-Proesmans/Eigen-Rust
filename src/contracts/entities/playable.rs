use std::cmp;
use std::fmt;
use std::hash;

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
pub trait IPlayable<'playable>
    : fmt::Debug + fmt::Display + IEntity<'playable> {
    // TODO; Implement
}

impl<'px> hash::Hash for IPlayable<'px> {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self._get_data_internal().hash(state);
    }
}

impl<'px> cmp::PartialEq for IPlayable<'px> {
    fn eq(
        &self,
        other: &IPlayable<'px>,
    ) -> bool {
        self._get_data_internal() == other._get_data_internal()
    }
}

impl<'px> cmp::Eq for IPlayable<'px> {}
