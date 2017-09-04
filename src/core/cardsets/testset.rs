use std::collections::HashMap;

use core::Card;
use enums::{ECardSets, EGameTags, ECardTypes};

// All card implementations MUST be implemented between the lazy_static! tags.
// The macro builts the object during runtime on first access.
// The static variable contains a reference to the implemented object.
// => obj: type = &something <=> ref obj: type = something <=
lazy_static! {

    pub static ref EX1_323H: Card = card! {
        id: "EX1_323h", // First argument MUST BE the card ID!
        name: "Lord Jaraxxus",
        kind: ECardTypes::Minion,
        set: ECardSets::Expert1,

        atk: 3,
        health: 15,
        cost: 9,

        // card_data can be set or not set..
        // EXPLICIT properties (like cost; atk) WILL ALWAYS override the value of the
        // matching EGameTag within card_data (see the debug output of this card for validation)!
        card_data: hashmap!{EGameTags::Battlecry => 0, EGameTags::Cost => 0},

        // These field can be optionally set if needed.. they can also be ommitted.
        entourage: None,
        play_requirements: None,
        reference_tags: Some(hashmap!{}),
        effects: None,
    };

    pub static ref FULL_SET: HashMap<&'static str, &'static Card> = hashmap!{
        "EX1_323h" => &*EX1_323H
    };

}
