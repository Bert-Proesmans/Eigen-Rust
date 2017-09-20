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
pub struct Hero {
    data: EntityData,
    card: &'static ICard,
    weapon_id: Option<u32>
}

impl fmt::Display for Hero {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "HERO [TODO]")
    }
}

impl Hero {
    pub fn weapon<'a>(
        &self,
        state: &'a GameManager,
    ) -> Option<&'a Weapon> {
        self.weapon_id.map(|id| Weapon::try_into(state.get_entity(id).unwrap()).unwrap())
    }

    pub fn set_weapon(
        &mut self,
        _state: &GameManager,
        weapon: &Weapon,
    ) -> EntityError::Result<()> {
        // Should we test for something before storing the weapon
        // id?
        self.weapon_id = Some(weapon.id());
        Ok(())
    }

    pub fn remove_weapon(
        &mut self,
        _state: &GameManager,
    ) -> EntityError::Result<()> {
        // Should we also move the weapon into the graveyard +
        // update all tags?
        match self.weapon_id {
            // Replace integer, if set, with none.
            Some(_) => self.weapon_id.take(),
            None => bail!(EntityError::ErrorKind::MissingEntity),
        };

        Ok(())
    }

    pub fn controller<'a>(
        &self,
        state: &'a GameManager,
    ) -> &'a Controller {
        let controller_entity = state.get_entity(self.controller_id()).expect("Bogus controller ID was set");
        Controller::try_into(controller_entity).unwrap()
    }
}


impl IEntity for Hero {
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
        Some(self)
    }

    fn as_character<'e>(&'e self) -> Option<&'e ICharacter> {
        Some(self)
    }

    fn as_any_mut<'e>(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut<'e>(&'e mut self) -> Option<&'e mut IPlayable> {
        Some(self)
    }

    fn as_character_mut<'e>(&'e mut self) -> Option<&'e mut ICharacter> {
        Some(self)
    }
}

impl IEntityInitializable for Hero {
    fn new(
        entity_id: u32,
        card: &'static ICard,
    ) -> EntityError::Result<Box<IEntity>> {
        let entity_data: EntityError::Result<_> =
            EntityData::from_data(entity_id, card._get_data_internal()).map_err(|e| e.into());
        let entity_data = try!(entity_data);

        let obj = Self {
            data: entity_data,
            card: card,
            weapon_id: None
        };

        // Box the object and return it to caller.
        Ok(Box::new(obj))
    }
}

impl IEntityCastable for Hero {
    fn try_into<'e>(_e: &'e IEntity) -> CastError::Result<&'e Self> {
        bail!(CastError::ErrorKind::NoCastProvided)
    }

    fn try_into_mut<'e>(_e: &'e mut IEntity) -> CastError::Result<&'e mut Self> {
        // cast_entity_mut!(e, (ECardTypes::Hero), Hero)
        bail!(CastError::ErrorKind::NoCastProvided)
    }
}

impl IPlayable for Hero {
    // add code here
}

impl ICharacter for Hero {
    // add code here
}
