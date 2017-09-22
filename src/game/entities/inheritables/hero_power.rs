use std::any::Any;
use std::fmt;

use contracts::cards::card::ICard;
use contracts::entities::character::ICharacter;
use contracts::entities::entity::IEntity;
use contracts::entities::entity_castable::IEntityCastable;
use contracts::entities::entity_data::IEntityData;
use contracts::entities::entity_initializable::IEntityInitializable;
use contracts::entities::playable::IPlayable;

use contracts::entities::entity_castable::errors as CastError;
use contracts::entities::errors as EntityError;

use game::entities::controller::Controller;
use game::entities::entity_data::EntityData;
use game::entities::inheritables::weapon::Weapon;

use game_manager::GameManager;

use enums::{ECardTypes, EGameTags};

#[derive(Debug)]
pub struct HeroPower {
    data: EntityData,
    card: &'static ICard
}

impl fmt::Display for HeroPower {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "HERO_POWER [TODO]")
    }
}

impl HeroPower {
    // add code here
}

impl IEntity for HeroPower {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal<'e, 'f: 'e>(&'e self) -> &'e (IEntityData + 'f) {
        &self.data
    }

    fn _get_data_internal_mut<'e, 'f: 'e>(&'e mut self) -> &'e mut (IEntityData + 'f) {
        &mut self.data
    }

    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        let mut tag_value = self.native_tag_value(tag);
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

    fn as_any<'e, 'f: 'e>(&'e self) -> &'e (Any + 'f) {
        self
    }

    fn as_playable<'e, 'f: 'e>(&'e self) -> Option<&'e (IPlayable + 'f)> {
        Some(self)
    }

    fn as_character<'e, 'f: 'e>(&'e self) -> Option<&'e (ICharacter + 'f)> {
        None
    }

    fn as_any_mut<'e, 'f: 'e>(&'e mut self) -> &'e mut (Any + 'f) {
        self
    }

    fn as_playable_mut<'e, 'f: 'e>(&'e mut self) -> Option<&'e mut (IPlayable + 'f)> {
        Some(self)
    }

    fn as_character_mut<'e, 'f: 'e>(&'e mut self) -> Option<&'e mut (ICharacter + 'f)> {
        None
    }
}

impl IEntityInitializable for HeroPower {
    fn new(
        entity_id: u32,
        card: &'static ICard,
    ) -> EntityError::Result<Box<IEntity>> {
        let entity_data: EntityError::Result<_> =
            EntityData::from_data(entity_id, card._get_data_internal()).map_err(|e| e.into());
        let entity_data = try!(entity_data);

        let obj = Self {
            data: entity_data,
            card: card
        };

        // Box the object and return it to caller.
        Ok(Box::new(obj))
    }
}

impl IEntityCastable for HeroPower {
    fn try_into<'e>(_e: &'e IEntity) -> CastError::Result<&'e Self> {
        // cast_entity!(e, (ECardTypes::HeroPower), HeroPower)
        bail!(CastError::ErrorKind::NoCastProvided);
    }

    fn try_into_mut<'e>(_e: &'e mut IEntity) -> CastError::Result<&'e mut Self> {
        // cast_entity_mut!(e, (ECardTypes::HeroPower), HeroPower)
        bail!(CastError::ErrorKind::NoCastProvided);
    }
}

impl IPlayable for HeroPower {
    // add code here
}
