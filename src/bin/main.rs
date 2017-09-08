extern crate eigen_rust;

use eigen_rust::contracts::models::ICardContainer;

use eigen_rust::core::cardsets::testset;
use eigen_rust::core::models::cardcontainer;
use eigen_rust::core::models::cardcontainer::CARDS;
use eigen_rust::enums::{ECardClasses, EFormats, EGameTags};

use eigen_rust::core::{Game, GameConfig};

fn main() {
    // Load cardsets from core.
    let ref set = testset::FULL_SET;
    println!("Amount of cards: {}", set.len());
    println!("-----------");

    let ref container = *cardcontainer::CARDS;
    println!("{:?}", container);
    println!("-----------");

    let test_card = container.from_id("EX1_323h").expect(
        "Card EX1_323h not found!",
    );
    println!("test-card: {:?}", test_card);
    println!("-----------");

    let tag_cost = test_card.tag_value(EGameTags::Cost);
    println!("Cost: {:?}", tag_cost);
    println!("-----------");

    let tag_unkn = test_card.tag_value(EGameTags::Timeout);
    println!("UNKN: {:?}", tag_unkn);
    println!("-----------");

    // Setup game config.
    let mut config = GameConfig::new();
    println!("GameConfig: {:?}", config);
    println!("-----------");


    config.game_format = Some(EFormats::Standard);
    config.player_names[0] = "P1";
    config.player_names[1] = "P2";
    config.player_heroclasses[0] = Some(ECardClasses::Rogue);
    config.player_heroclasses[1] = Some(ECardClasses::Mage);
    // Remove when there are more cards implemented
    config.build_heroes = false;
    config.build_hero_powers = false;

    config.player_decks[0] = Some(vec![
        CARDS.from_name("Lord Jaraxxus")
        .expect(
            "Card Lord Jaraxxus \
             not found!"
        ),
    ]);


    println!("GameConfig UPDATED: {:?}", config);
    println!("-----------");

    // Setup game.

    let mut game = Game::new(config);
}
