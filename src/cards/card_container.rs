use std::collections::HashMap;
use std::fmt;

use contracts::cards::card::ICard;
use contracts::cards::card_container::ICardContainer;
use contracts::cards::errors::*;

use card_sets::test_set;

use cards::card::Card;

use enums::{ECardClasses, ECardSets, ECardTypes};

pub const STANDARD_SET: [ECardSets; 7] = [ECardSets::Core,
                                          ECardSets::Expert1,
                                          ECardSets::Og,
                                          ECardSets::Kara,
                                          ECardSets::Gangs,
                                          ECardSets::Ungoro,
                                          ECardSets::Icecrown];

pub const WILD_SET: [ECardSets; 13] = [ECardSets::Core,
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
                                       ECardSets::Tgt];

lazy_static! {
    // The ONE container for all cards!
    pub static ref CARDS: CardContainer<'static> = CardContainer::new();

    // Card used to derive Game entities!
    pub static ref GAME_CARD: Card<'static> = card_novalidate! {
        dbf_id: 0,
        card_id: "GAME",
        name: "Game",
        kind: ECardTypes::Game,
        set: ECardSets::TestTemporary // TODO; Change this to no-set after macro update!
    };

    // Card used to derive Controller entities!
    pub static ref CONTROLLER_CARD: Card<'static> = card_novalidate! {
        dbf_id: 0,
        card_id: "PLAYER",
        name: "Player",
        kind: ECardTypes::Player,
        set: ECardSets::TestTemporary // TODO; Change this to no-set after macro update!
    };
}

#[derive(Debug)]
pub struct CardContainer<'container> {
    all_cards: Option<HashMap<u32, &'container (ICard<'static> + 'static)>>,

    all_collectible_standard: Option<Vec<&'container (ICard<'static> + 'static)>>,
    all_collectible_wild: Option<Vec<&'container (ICard<'static> + 'static)>>,

    all_collectible_standard_per_class: Option<HashMap<ECardClasses, Vec<&'container (ICard<'static> + 'static)>>>,
    all_collectible_wild_per_class: Option<HashMap<ECardClasses, Vec<&'container (ICard<'static> + 'static)>>>
}

impl<'cx> fmt::Display for CardContainer<'cx> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "CONTAINER [TODO]")
    }
}

impl CardContainer<'static> {
    fn new() -> CardContainer<'static> {
        // Create empty container.
        let container = CardContainer {
            all_cards: None,
            all_collectible_standard: None,
            all_collectible_wild: None,
            all_collectible_standard_per_class: None,
            all_collectible_wild_per_class: None
        };

        // Fill all container fields.
        container.collect_cards()
    }

    fn collect_cards(self) -> Self {
        let mut card_data = HashMap::new();

        // PROVISION ALL CARD SETS HERE

        // Test set
        for (key, value) in test_set::FULL_SET.iter() {
            // Only this line needs to be adjusted if a different
            // return type for
            // Card is necessary!
            card_data.insert(*key, *value as &ICard);
        }

        self.construct_collections(card_data)
    }

    fn construct_collections(
        mut self,
        all_cards: HashMap<u32, &'static ICard>,
    ) -> Self {
        // Create collections for each format.

        let collectible = all_cards.iter()
                            .filter(|&(_,v)| v.is_collectible())
                            .map(|(_,&v)| v) // Deref reference from iterator.
                            .collect::<Vec<_>>();

        // Set map of all cards.
        self.all_cards = Some(all_cards);

        let collectible_standard = collectible.iter()
                                    .filter(|v| STANDARD_SET.contains(&v.card_set().expect("Card without set")))
                                    .map(|&v| v) // Deref reference from iterator.
                                    .collect::<Vec<_>>();
        self.all_collectible_standard = Some(collectible_standard);

        let collectible_wild = collectible.iter()
                                .filter(|v| WILD_SET.contains(&v.card_set().expect("Card without set")))
                                .map(|&v| v) // Deref reference from iterator.
                                .collect::<Vec<_>>();
        self.all_collectible_wild = Some(collectible_wild);


        // Create containers for each class AND format!
        // TODO

        self
    }
}

impl<'cx> ICardContainer<'cx> for CardContainer<'cx> {
    fn all_cards(&self) -> &HashMap<u32, &'static ICard> {
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

    fn from_dbf_id(
        &self,
        id: u32,
    ) -> Result<&'static ICard> {
        self.all_cards.as_ref().unwrap().get(&id).map(|&x| x).ok_or(Error::from_kind(ErrorKind::UnknownDBFID(id)))
    }

    fn from_name(
        &self,
        name: &str,
    ) -> Result<&'static ICard> {
        match self.all_cards.as_ref().unwrap()
                .iter()
                .filter(|&(_,v)| v.name().starts_with(name))
                .map(|(_,&v)| v) // Deref reference from iterator.
                .take(1)
                .next() {
            Some(card) => Ok(card),
            _ => bail!(ErrorKind::UnknownName(name.to_owned())),
        }
    }

    fn hero_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>> {
        let set = self.all_cards.as_ref().unwrap()
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
            .collect::<Vec<_>>();

        if set.is_empty() {
            bail!(ErrorKind::EmptyClassMatch(class));
        } else {
            Ok(set)
        }
    }

    fn hero_power_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>> {
        let set = self.all_cards.as_ref().unwrap()
            .iter()
            .filter(|&(_,v)| match v.card_type() {
                Some(t) => t == ECardTypes::HeroPower,
                None => false,
            })
            .filter(|&(_, v)| match v.card_class() {
                Some(c) => c == class,
                None => false,
            })
            .map(|(_, &v)| v) // Take only the reference to the card.
            .collect::<Vec<_>>();

        if set.is_empty() {
            bail!(ErrorKind::EmptyClassMatch(class));
        } else {
            Ok(set)
        }
    }
}
