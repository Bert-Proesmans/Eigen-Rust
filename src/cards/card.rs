use std::collections::HashMap;
use std::fmt;

use contracts::cards::card::ICard;
// use contracts::cards::errors::*;

use enums::{ECardSets, ECardTypes, EGameTags, EPlayRequirements};

#[derive(Debug, Default)]
pub struct Card {
    id: &'static str,
    pub name: &'static str,

    // Card specifics; need hardcoding during construction!
    pub kind: ECardTypes,
    pub set: ECardSets,
    pub atk: u32,
    pub health: u32,
    pub cost: u32,
    // TODO; Add more explicit card properties!
    //
    // Some cards need the following fields implemented.
    pub entourage: Option<Vec<&'static str>>,
    pub play_requirements: Option<HashMap<EPlayRequirements, u32>>,
    pub reference_tags: Option<HashMap<EGameTags, u32>>,
    pub effects: Option<Vec<Box<fmt::Debug>>>,
    pub card_data: HashMap<EGameTags, u32>
}

// Force implement the Sync trait since we guarantee
// that no edits after construction are allowed!
unsafe impl Sync for Card {}

impl Card {
    pub fn new(id: &'static str) -> Self {
        Self {
            id: id,

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
        // Test enumeration values to not have their respective
        // default keys (= ::Invalid)!
        if self.kind == ECardTypes::default() {
            println!("card \"{:?}\" has default value for `kind`: {:?}", "ID", "KIND");
        }

        if self.set == ECardSets::default() {
            println!("card \"{:?}\" has default value for `set`: {:?}", "ID", "KIND");
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

impl fmt::Display for Card {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "CARD [TODO]")
    }
}


impl ICard for Card {
    fn id(&self) -> &'static str {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn _get_data_internal(&self) -> &HashMap<EGameTags, u32> {
        &self.card_data
    }

    // +'static -> underlying object is NOT a reference (which
    // typically has a lower lifetime duration).
    fn effects(&self) -> Option<&Vec<Box<fmt::Debug + 'static>>> {
        self.effects.as_ref()
    }
}
