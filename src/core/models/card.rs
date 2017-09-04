use std::fmt;
use std::collections::HashMap;

use contracts::models::ICard;
use contracts::effects::IEffect;

use enums::{EGameTags, EPlayRequirements, ECardSets, ERarities, ECardClasses, ECardTypes};

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

    // Some cards need the following fields implemented.
    pub entourage: Option<Vec<&'static str>>,
    pub play_requirements: Option<HashMap<EPlayRequirements, u32>>,
    pub reference_tags: Option<HashMap<EGameTags, u32>>,
    pub effects: Option<Vec<Box<IEffect + 'static>>>,
    pub card_data: HashMap<EGameTags, u32>,
}

/*
 * Force implement the Sync trait since we guarantee ourselves that we won't
 * mutate the contents of card_data_internal concurrent within multiple threads.
 * -> Except for the finalize method we'll NEVER edit the contents of card_data_internal!
 */
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
            ..Default::default()
        }
    }

    fn rarity(&self) -> Option<ERarities> {
        // This is pretty nasty; we have to wait for a better method.
        ERarities::from(self.tag_value(EGameTags::Rarity))
    }

    pub fn validate(self) -> Self {
        // Test enumeration values to not have their respective default keys (= ::Invalid)!
        if self.kind == ECardTypes::default() {
            panic!(
                "card \"{:}\" has default value for `kind`: {:?}",
                self.id,
                self.kind
            );
        }

        if self.set == ECardSets::default() {
            panic!(
                "card \"{:}\" has default value for `set`: {:?}",
                self.id,
                self.set
            );
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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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

    fn card_type(&self) -> Option<ECardTypes> {
        ECardTypes::from(self.tag_value(EGameTags::Cardtype))

    }

    fn tag_value(&self, tag: EGameTags) -> u32 {
        self.card_data.get(&tag) // Get the tag value for key
            .map(|value| *value) // Deref (through copy) the returned &u32
            .unwrap_or(0) // Fallback to 0 if None was discovered!
    }

    fn has_implemented_effects(&self) -> bool {
        // Implemented if the effects variable contains a container which is
        // NON-EMPTY!
        if let Some(ref container) = self.effects {
            return container.len() > 0;
        }

        // If effects variable contains None, it's considered implemented!
        // This might be counter-intuitive, but an empty Vec is used as None variant.
        // effects: None is explicitly set if the card has no effects.
        true
    }

    fn effects(&self) -> Option<&Vec<Box<IEffect + 'static>>> {
        match self.effects {
            Some(ref expr) => Some(expr),
            None => None,
        }
    }

    fn is_collectible(&self) -> bool {
        self.tag_value(EGameTags::Collectible) > 0
    }

    fn card_set(&self) -> Option<ECardSets> {
        ECardSets::from(self.tag_value(EGameTags::CardSet))

    }

    fn card_class(&self) -> Option<ECardClasses> {
        ECardClasses::from(self.tag_value(EGameTags::Class))
    }

    fn has_combo(&self) -> bool {
        self.tag_value(EGameTags::Combo) > 0
    }

    fn card_cost(&self) -> u32 {
        self.tag_value(EGameTags::Cost)
    }

    fn has_overload(&self) -> bool {
        self.tag_value(EGameTags::Overload) > 0
    }

    fn overload(&self) -> u32 {
        self.tag_value(EGameTags::OverloadOwed)
    }

    fn max_allowed_in_deck(&self) -> u32 {
        match self.rarity() {
            Some(ERarities::Legendary) => 1,
            _ => 2,
        }
    }
}
