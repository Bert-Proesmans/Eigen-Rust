use std::collections::HashMap;

use cards::card::Card;

use enums::{ECardSets, ECardTypes, EGameTags};

// TODO; Rework the set implementation when compiler
// plugins are stable.
// We can use crate rust_phf for compile time hashmap
// generation.

// All card implementations MUST be implemented between the
// lazy_static! tags.
// The macro builts the object during runtime on first
// access.
// The static variable contains a reference to the
// implemented object.
// => obj: type = &something <=> ref obj: type = something
// <=
lazy_static! {

    pub static ref EX1_323H: Card = card! {
        id: "EX1_323h", // First argument MUST BE the card ID!
        name: "Lord Jaraxxus",  // Name in the default language (English), i'm thinking about having
                                // the possibility to extend cardcontainer to deliver translated
                                // cardnames to the wrapping code.
        kind: ECardTypes::Minion,
        set: ECardSets::Expert1,

        atk: 3,
        health: 15,
        cost: 9,

        // card_data can be set or not set..
        // EXPLICIT properties (like cost; atk) WILL ALWAYS override the value of the
        // matching EGameTag within card_data (see the debug output of this card for validation)!
        card_data: hashmap!{EGameTags::Battlecry => 0, EGameTags::Cost => 0},

        // These fields can be optionally set if needed.. they can also be ommitted.
        entourage: None,
        play_requirements: None,
        reference_tags: Some(hashmap!{}),
        effects: None,
    };

    // Collect all cards of the set into this hashmap.
    pub static ref FULL_SET: HashMap<&'static str, &'static Card> = hashmap!{
        "EX1_323h" => &*EX1_323H
    };

}

#[cfg(test)]
mod tests {

    use super::*;
    use cards::card_container::CARDS;

    #[test]
    fn load_cardset() {
        let ref set = testset::FULL_SET;
        println!("Amount of cards: {}", set.len());
        println!("-----------");
    }

    #[test]
    fn load_container() {
        let ref container = *CARDS;
        println!("{:?}", container);
        println!("-----------");
    }

    #[test]
    fn load_test_card() {
        let ref container = *cardcontainer::CARDS;
        let test_card = container.from_id("EX1_323h").expect("Card EX1_323h not found!");

        println!("test-card: {:?}", test_card);
        println!("-----------");

        let tag_cost = test_card.tag_value(EGameTags::Cost);
        println!("Cost: {:?}", tag_cost);
        println!("-----------");

        let tag_unkn = test_card.tag_value(EGameTags::Timeout);
        println!("UNKN: {:?}", tag_unkn);
        println!("-----------");
    }
}
