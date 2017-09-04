
use std::collections::HashMap;

use contracts::models::ICard;
use enums::ECardClasses;

pub trait ICardContainer {
    fn all_cards(&self) -> &HashMap<&'static str, &'static ICard>;

    fn all_wild(&self) -> &Vec<&'static ICard>;
    fn all_standard(&self) -> &Vec<&'static ICard>;

    fn wild(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;
    fn standard(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;

    fn from_id(&self, id: &str) -> Option<&'static ICard>;
    fn from_name(&self, name: &str) -> Option<&'static ICard>;

    fn hero_cards(&self, class: ECardClasses) -> Vec<&'static ICard>;
}
