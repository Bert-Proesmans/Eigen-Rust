extern crate eigen_rust;

use eigen_rust::contracts::models::ICardContainer;

use eigen_rust::core::cardsets::testset;
use eigen_rust::core::models::cardcontainer;
use eigen_rust::enums::EGameTags;

fn main() {
    // Load cardsets from core.
    let ref set = testset::FULL_SET;
    println!("Amount of cards: {}", set.len());
    println!("-----------");

    let ref container = *cardcontainer::CARDS;
    println!("{:?}", container);
    println!("-----------");

    let test_card = container.from_id("EX1_323h").unwrap();
    println!("test-card: {:?}", test_card);
    println!("-----------");

    let tag_cost = test_card.tag_value(EGameTags::Cost);
    println!("Cost: {:?}", tag_cost);
    println!("-----------");

    let tag_unkn = test_card.tag_value(EGameTags::Timeout);
    println!("UNKN: {:?}", tag_unkn);
    println!("-----------");

    // Setup game config.

    // Setup game.
}
