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
pub trait IPlayable: fmt::Debug + fmt::Display + IEntity {
    // TODO; Implement
}

impl<'a> hash::Hash for IPlayable + 'a {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self._get_data_internal().hash(state);
    }
}

impl<'a> cmp::PartialEq for IPlayable + 'a {
    fn eq(
        &self,
        other: &IPlayable,
    ) -> bool {
        self._get_data_internal() == other._get_data_internal()
    }
}

impl<'a> cmp::Eq for IPlayable + 'a {}
