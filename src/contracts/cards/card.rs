use std::collections::HashMap;
use std::fmt;

use num_traits::FromPrimitive;

use contracts::effects::card_effect::ICardEffect;

use enums::{ECardClasses, ECardSets, ECardTypes, EGameTags, ERarities};

/// Representation of a card in the game
pub trait ICard<'card>: fmt::Debug + fmt::Display + Sync {
    /// Returns Unique Identifier #1; the database ID of
    /// the card
    fn dbf_id(&self) -> u32;

    /// Returns Unique Identifier #2; the stringified ID of
    /// the card
    fn card_id(&self) -> &'card str;

    /// Returns the name of the card
    fn name(&self) -> &'card str;

    /// Returns a map of properties assigned to this card
    fn _get_data_internal(&self) -> &HashMap<EGameTags, u32>;

    /// Returns the list of effects caused by interacting
    /// with this card
    // +'static -> underlying object is NOT a reference (which
    // typically has a lower lifetime duration).
    fn effects(&self) -> Option<&Vec<Box<ICardEffect<'card> + 'card>>>;

    /// Returns a boolean indicating if this card has
    /// implemented effects
    ///
    /// Implementing effects is optional, so calculating
    /// the result value
    /// must take this into consideration.
    /// If self.effects() returns None or Some([non-empty
    /// vec]) this method
    /// returns true.
    /// If self.effects() returns Some([empty vec]) this
    /// method returns false.
    fn has_implemented_effects(&self) -> bool {
        if let Some(effects_vec) = self.effects() {
            return effects_vec.len() > 0;
        }

        true
    }

    /// Returns the value for the requested property within
    /// the data
    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        self._get_data_internal()
                    .get(&tag)
                    // Deref primitive value if existing.
                    .map(|&v| v)
                    // Return 0 if the tag wasn't found in the data.
                    .unwrap_or(0)
    }

    /////////////////////////////////////////////////////////////
    // Most used tags on each card have their shortcuts below.
    // //
    // //////////////////////////////////////////////////////////

    /// Returns the rarity of this card
    fn rarity(&self) -> Option<ERarities> {
        self._get_data_internal()
                    .get(&EGameTags::Rarity)
                    // Dereference returned int and build the proper enum value.
                    .and_then(|&v| ERarities::from_u32(v))
    }

    /// Returns the type of entity this card produces
    fn card_type(&self) -> Option<ECardTypes> {
        self._get_data_internal().get(&EGameTags::Cardtype).and_then(|&v| ECardTypes::from_u32(v))
    }

    /// Returns if this card is a collectible one
    fn is_collectible(&self) -> bool {
        self.tag_value(EGameTags::Collectible) > 0
    }

    /// Returns the set this card belongs to
    fn card_set(&self) -> Option<ECardSets> {
        self._get_data_internal().get(&EGameTags::CardSet).and_then(|&v| ECardSets::from_u32(v))
    }

    /// Returns the class this card belongs to
    fn card_class(&self) -> Option<ECardClasses> {
        self._get_data_internal().get(&EGameTags::Class).and_then(|&v| ECardClasses::from_u32(v))
    }

    /// Returns whether this card has the combo mechanic
    fn has_combo(&self) -> bool {
        self.tag_value(EGameTags::Combo) > 0
    }

    /// Returns the resource cost of this card
    fn card_cost(&self) -> u32 {
        self.tag_value(EGameTags::Cost)
    }

    /// Returns whether this card introduces resource
    /// overload
    fn has_overload(&self) -> bool {
        self.tag_value(EGameTags::Overload) > 0
    }

    /// Returns the amount of resource overload introduced
    /// by this card
    fn overload_amount(&self) -> u32 {
        self.tag_value(EGameTags::OverloadOwed)
    }

    /// Returns the maximum allowed amount of times this
    /// card is allowed into a deck
    fn max_allowed_in_deck(&self) -> u32 {
        match self.rarity() {
            // Only legendaries are constrained.
            Some(ERarities::Legendary) => 1,
            // Default to 2 allowed copies of the same card.
            _ => 2,
        }
    }
}
