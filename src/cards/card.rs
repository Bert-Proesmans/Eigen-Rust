use std::collections::{HashMap, HashSet};
use std::fmt;
use std::slice::Iter;

use contracts::cards::card::ICard;
use contracts::effects::card_effect::ICardEffect;
// use contracts::cards::errors::*;

// use effects::card_effect::CardEffect;

use game_manager::GameManager;

use enums::{ECardSets, ECardTypes, EGameTags, EPlayRequirements};

#[derive(Debug, Default)]
pub struct Card<'card> {
    dbf_id: u32,
    card_id: &'card str,
    pub name: &'card str,

    // Card specifics; need hardcoding during construction!
    pub kind: ECardTypes,
    pub set: ECardSets,
    pub atk: u32,
    pub health: u32,
    pub cost: u32,
    // TODO; Add more explicit card properties!
    //
    // Some cards need the following fields implemented.
    pub entourage: Option<Vec<&'card str>>,
    pub play_requirements: Option<HashMap<EPlayRequirements, u32>>,
    pub reference_tags: Option<HashSet<EGameTags>>,
    // TODO; Change to ICardEffect
    pub effects: Option<Vec<Box<ICardEffect>>>,
    pub card_data: HashMap<EGameTags, u32>
}

// Force implement the Sync trait since we guarantee
// that no edits after construction are allowed!
unsafe impl<'cx> Sync for Card<'cx> {}

impl<'cx> Card<'cx> {
    pub fn new(
        dbf_id: u32,
        card_id: &'cx str,
    ) -> Self {
        Self {
            dbf_id: dbf_id,
            card_id: card_id,

            // Default WILL build empty collection objects!
            // So we explicitly set most options to None
            entourage: None,
            play_requirements: None,
            reference_tags: None,
            effects: None,

            // We always want a default card_data hashmap because it's guaranteed
            // to be used!
            //
            // Use default values for non provided fields.
            ..Default::default()
        }
    }

    pub fn validate(self) -> Self {

        if self.dbf_id == 0 {
            panic!("card `{:?}` has default value for `dbf_id`: 0", self.card_id);
        }

        // Test enumeration values to not have their respective
        // default keys (= ::Invalid)!
        if self.kind == ECardTypes::default() {
            panic!("card `{:?}` has default value for `kind`: {:?}", self.card_id, self.kind);
        }

        if self.set == ECardSets::default() {
            panic!("card `{:?}` has default value for `set`: {:?}", self.card_id, self.kind);
        }

        self
    }

    pub fn finalize(mut self) -> Self {
        {
            // Push all public card fields into the internal hashmap.
            let ref mut container = self.card_data;

            container.insert(EGameTags::Cardtype, self.kind.clone() as u32);
            container.insert(EGameTags::CardSet, self.set.clone() as u32);
            if self.atk > 0 {
                container.insert(EGameTags::Atk, self.atk.clone() as u32);
            }
            if self.health > 0 {
                container.insert(EGameTags::Health, self.health.clone() as u32);
            }
            if self.cost > 0 {
                container.insert(EGameTags::Cost, self.cost.clone() as u32);
            }
        } // End borrow self.card_data

        self
    }
}

impl<'cx> fmt::Display for Card<'cx> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "CARD [TODO]")
    }
}


impl<'cx> ICard for Card<'cx> {
    fn dbf_id(&self) -> u32 {
        self.dbf_id
    }

    fn card_id(&self) -> &str {
        self.card_id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn _get_data_internal(&self) -> &HashMap<EGameTags, u32> {
        &self.card_data
    }

    fn effects(&self) -> Option<Iter<Box<ICardEffect>>> {
        if let Some(ref effects) = self.effects {
            return Some(effects.iter());
        }

        None
    }
}
