use std::collections::HashMap;
use std::fmt;

use num_traits::FromPrimitive;

use enums::{ECardClasses, ECardSets, ECardTypes, EGameTags, ERarities};

// use super::errors::*;

pub trait ICard: fmt::Debug + fmt::Display + Sync {
    fn dbf_id(&self) -> u32;
    fn card_id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn _get_data_internal(&self) -> &HashMap<EGameTags, u32>;
    // +'static -> underlying object is NOT a reference (which
    // typically has a lower lifetime duration).
    fn effects(&self) -> Option<&Vec<Box<fmt::Debug>>>;

    fn has_implemented_effects(&self) -> bool {
        // Having an explicit None value for effects means there
        // are NO effects to be implemented!
        // -> returns TRUE
        // Having a vector which holds effect objects also returns
        // TRUE
        // Having a vector which is empty means the card is
        // awaiting implementation! -> returns FALSE

        if let Some(effects_vec) = self.effects() {
            return effects_vec.len() > 0;
        }

        true
    }

    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        self._get_data_internal()
                    .get(&tag)
                    // Deref primitive value if existing.
                    .map(|&v| v)
                    // Return 0 is the tag wasn't found in the data.
                    .unwrap_or(0)
    }

    /////////////////////////////////////////////////////////////
    // Most used tags on each card have their shortcuts below.
    // //
    // //////////////////////////////////////////////////////////

    fn rarity(&self) -> Option<ERarities> {
        self._get_data_internal()
                    .get(&EGameTags::Rarity)
                    // Dereference returned int and build the proper enum value.
                    .and_then(|&v| ERarities::from_u32(v))
    }

    fn card_type(&self) -> Option<ECardTypes> {
        self._get_data_internal().get(&EGameTags::Cardtype).and_then(|&v| ECardTypes::from_u32(v))
    }

    fn is_collectible(&self) -> bool {
        self.tag_value(EGameTags::Collectible) > 0
    }

    fn card_set(&self) -> Option<ECardSets> {
        self._get_data_internal().get(&EGameTags::CardSet).and_then(|&v| ECardSets::from_u32(v))
    }

    fn card_class(&self) -> Option<ECardClasses> {
        self._get_data_internal().get(&EGameTags::Class).and_then(|&v| ECardClasses::from_u32(v))
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

    fn overload_amount(&self) -> u32 {
        self.tag_value(EGameTags::OverloadOwed)
    }

    fn max_allowed_in_deck(&self) -> u32 {
        match self.rarity() {
            // Only legendaries are constrained.
            Some(ERarities::Legendary) => 1,
            // Default to 2 allowed copies of the same card.
            _ => 2,
        }
    }
}
