use std::any::Any;
// use std::borrow::{Borrow, BorrowMut};
// use std::collections::HashMap;

use std::fmt;

use contracts::cards::card::ICard;
use contracts::entities::character::ICharacter;
use contracts::entities::entity::IEntity;
use contracts::entities::entity_data::IEntityData;
use contracts::entities::errors::*;
use contracts::entities::playable::IPlayable;

use cards::card_container::GAME_CARD;
use game::entities::entity_data::EntityData;
use game_manager::GAME_ENTITY_ID;

use enums::{EGameTags, EZones};

#[derive(Debug)]
pub struct Game {
    data: EntityData,
    card: &'static ICard
}

impl fmt::Display for Game {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "GAME [TODO]")
    }
}

impl IEntity for Game {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal(&self) -> &IEntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut IEntityData {
        &mut self.data
    }

    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        let tag_value = self.native_tag_value(tag); // MUT
        // TODO; process all aura's and other stuff which influence
        // this tag.

        tag_value
    }

    fn as_any<'e>(&'e self) -> &'e Any {
        self
    }

    fn as_playable<'e>(&'e self) -> Option<&'e IPlayable> {
        None
    }

    fn as_character<'e>(&'e self) -> Option<&'e ICharacter> {
        None
    }

    fn as_any_mut<'e>(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut<'e>(&'e mut self) -> Option<&'e mut IPlayable> {
        None
    }

    fn as_character_mut<'e>(&'e mut self) -> Option<&'e mut ICharacter> {
        None
    }
}

impl Game {
    pub fn new() -> Result<Self> {
        let game_entity_data: Result<_> =
            EntityData::from_data(GAME_ENTITY_ID, &GAME_CARD.card_data).map_err(|e| e.into());
        let mut game_entity_data = try!(game_entity_data);

        // Push additional entity information for this game
        game_entity_data.set_tag(EGameTags::Zone, EZones::Play as u32);

        Ok(Self {
            data: game_entity_data,
            card: &*GAME_CARD
        })
    }
}