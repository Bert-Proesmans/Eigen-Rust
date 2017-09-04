
use std::collections::HashMap;

use contracts::models::{ICard, ICardContainer};
use core::cardsets;
use core::Card;

use enums::{ECardSets, ECardClasses, ECardTypes, EGameTags};

pub const STANDARD_SET: [ECardSets; 7] = [
    ECardSets::Core,
    ECardSets::Expert1,
    ECardSets::Og,
    ECardSets::Kara,
    ECardSets::Gangs,
    ECardSets::Ungoro,
    ECardSets::Icecrown,
];

pub const WILD_SET: [ECardSets; 13] = [
    ECardSets::Core,
    ECardSets::Expert1,
    ECardSets::Og,
    ECardSets::Kara,
    ECardSets::Gangs,
    ECardSets::Ungoro,
    ECardSets::Icecrown,
    // END STANDARD
    ECardSets::Brm,
    ECardSets::Gvg,
    ECardSets::Hof,
    ECardSets::Naxx,
    ECardSets::Loe,
    ECardSets::Tgt,
];

lazy_static! {
    // The ONE container for all cards!
    pub static ref CARDS: CardContainer = CardContainer::new();

    // Card used to derive Game entities!
    pub static ref GAME_CARD: Card = card! {
        id: "GAME",
        name: "Game",
        kind: ECardTypes::Game,
        set: ECardSets::NoSet
    };

    // Card used to derive Controller entities!
    pub static ref CONTROLLER_CARD: Card = card! {
        id: "PLAYER",
        name: "Player",
        kind: ECardTypes::Player,
        set: ECardSets::NoSet
    };
}

#[derive(Debug)]
pub struct CardContainer {
    all_cards: Option<HashMap<&'static str, &'static ICard>>,

    all_collectible_standard: Option<Vec<&'static ICard>>,
    all_collectible_wild: Option<Vec<&'static ICard>>,

    all_collectible_standard_per_class: Option<HashMap<ECardClasses, Vec<&'static ICard>>>,
    all_collectible_wild_per_class: Option<HashMap<ECardClasses, Vec<&'static ICard>>>,
}

impl CardContainer {
    fn new() -> CardContainer {
        // Create empty container.
        let container = CardContainer {
            all_cards: None,
            all_collectible_standard: None,
            all_collectible_wild: None,
            all_collectible_standard_per_class: None,
            all_collectible_wild_per_class: None,
        };

        // Fill all container fields.
        container.collect_cards()
    }

    fn collect_cards(self) -> Self {
        let mut card_data = HashMap::new();

        /* PROVISION ALL CARD SETS HERE */

        // Test set
        for (key, value) in cardsets::testset::FULL_SET.iter() {
            // Only this line needs to be adjusted if a different return type for
            // Card is necessary!
            card_data.insert(*key, *value as &ICard);
        }

        self.construct_collections(card_data)
    }

    fn construct_collections(mut self, all_cards: HashMap<&'static str, &'static ICard>) -> Self {
        // Create collections for each format.

        let collectible = all_cards.iter()
                            .filter(|&(_,v)| v.is_collectible())
                            .map(|(_,&v)| v) // Deref reference from iterator.
                            .collect::<Vec<_>>();

        // Set map of all cards.
        self.all_cards = Some(all_cards);

        let collectible_standard = collectible.iter()
                                    .filter(|v| STANDARD_SET.contains(&v.card_set().unwrap()))
                                    .map(|&v| v) // Deref reference from iterator.
                                    .collect::<Vec<_>>();
        self.all_collectible_standard = Some(collectible_standard);

        let collectible_wild = collectible.iter()
                                .filter(|v| WILD_SET.contains(&v.card_set().unwrap()))
                                .map(|&v| v) // Deref reference from iterator.
                                .collect::<Vec<_>>();
        self.all_collectible_wild = Some(collectible_wild);


        // Create containers for each class AND format!
        // TODO

        self
    }
}

impl ICardContainer for CardContainer {
    fn all_cards(&self) -> &HashMap<&'static str, &'static ICard> {
        self.all_cards.as_ref().unwrap()
    }

    fn all_wild(&self) -> &Vec<&'static ICard> {
        self.all_collectible_wild.as_ref().unwrap()
    }
    fn all_standard(&self) -> &Vec<&'static ICard> {
        self.all_collectible_standard.as_ref().unwrap()
    }

    fn wild(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>> {
        self.all_collectible_wild_per_class.as_ref().unwrap()
    }
    fn standard(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>> {
        self.all_collectible_standard_per_class.as_ref().unwrap()
    }

    fn from_id(&self, id: &str) -> Option<&'static ICard> {
        match self.all_cards.as_ref().unwrap().get(id) {
            Some(&item) => Some(item),
            None => None,
        }
    }
    fn from_name(&self, name: &str) -> Option<&'static ICard> {
        self.all_cards.as_ref().unwrap()
                .iter()
                .filter(|&(_,v)| v.name().starts_with(name))
                .map(|(_,&v)| v) // Deref reference from iterator.
                .take(1)
                .next()
    }

    fn hero_cards(&self, class: ECardClasses) -> Vec<&'static ICard> {
        self.all_cards.as_ref().unwrap()
            .iter()
            .filter(|&(_, v)| match v.card_type() {
                Some(t) => t == ECardTypes::Hero,
                None => false,
            })
            .filter(|&(_, v)| match v.card_class() {
                Some(c) => c == class,
                None => false,
            })
            .map(|(_, &v)| v) // Take only the reference to the card.
            .collect::<Vec<_>>()
    }
}
