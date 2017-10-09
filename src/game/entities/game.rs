use std::any::Any;
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
pub struct Game<'game> {
    data: EntityData,
    card: &'game ICard<'game>
}

impl<'gx> fmt::Display for Game<'gx> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "GAME [TODO]")
    }
}

impl<'ex> IEntity<'ex> for Game<'ex> {
    fn reference_card(&self) -> &ICard<'ex> {
        self.card
    }

    fn _get_data_internal(&self) -> &(IEntityData + 'ex) {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut (IEntityData + 'ex) {
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

    fn set_tag_value(
        &mut self,
        _tag: EGameTags,
        _value: u32,
    ) -> Option<u32> {
        None
    }

    fn as_any(&self) -> &(Any + 'ex) {
        self
    }

    fn as_playable(&self) -> Option<&IPlayable<'ex>> {
        None
    }

    fn as_character(&self) -> Option<&ICharacter<'ex>> {
        None
    }

    fn as_any_mut(&mut self) -> &mut (Any + 'ex) {
        self
    }

    fn as_playable_mut(&mut self) -> Option<&mut IPlayable<'ex>> {
        None
    }

    fn as_character_mut(&mut self) -> Option<&mut ICharacter<'ex>> {
        None
    }
}

impl<'gx> Game<'gx> {
    pub fn new() -> Result<Self> {
        let game_entity_data: Result<_> =
            EntityData::from_data(GAME_ENTITY_ID, &GAME_CARD.card_data).map_err(|e| e.into());
        let mut game_entity_data = try!(game_entity_data);

        // Push additional entity information for this game
        game_entity_data.set_tag(EGameTags::Zone, EZones::Play as u32);

        (&*GAME_CARD).DoFunc();

        Ok(Self {
            data: game_entity_data,
            card: &*GAME_CARD
        })
    }
}
