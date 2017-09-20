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

// use game::entities::controller::Controller;

use game::entities::entity_data::EntityData;

use enums::{ECardTypes, EGameTags};

#[derive(Debug)]
pub struct Weapon {
    data: EntityData,
    card: &'static ICard
}

impl fmt::Display for Weapon {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "WEAPON [TODO]")
    }
}

impl Weapon {
    // add code here
}

impl IEntity for Weapon {
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
        let mut tag_value = self.native_tag_value(tag);
        // TODO; process all aura's and other stuff which influence
        // this tag.

        tag_value
    }

    fn as_any<'e>(&'e self) -> &'e Any {
        self
    }

    fn as_playable<'e>(&'e self) -> Option<&'e IPlayable> {
        Some(self)
    }

    fn as_character<'e>(&'e self) -> Option<&'e ICharacter> {
        None
    }

    fn as_any_mut<'e>(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut<'e>(&'e mut self) -> Option<&'e mut IPlayable> {
        Some(self)
    }

    fn as_character_mut<'e>(&'e mut self) -> Option<&'e mut ICharacter> {
        None
    }
}

impl IEntityInitializable for Weapon {
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

impl IEntityCastable for Weapon {
    fn try_into<'e>(_e: &'e IEntity) -> CastError::Result<&'e Self> {
        // cast_entity!(e, (ECardTypes::Weapon), Weapon)
        bail!(CastError::ErrorKind::NoCastProvided);
    }

    fn try_into_mut<'e>(_e: &'e mut IEntity) -> CastError::Result<&'e mut Self> {
        // cast_entity_mut!(e, (ECardTypes::Weapon), Weapon)
        bail!(CastError::ErrorKind::NoCastProvided);
    }
}

impl IPlayable for Weapon {
    // add code here
}
