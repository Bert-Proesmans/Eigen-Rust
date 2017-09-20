use std::collections::HashMap;
use std::fmt::{Debug, Display};

use contracts::cards::card::ICard;
use enums::ECardClasses;

use super::errors::*;

pub trait ICardContainer: Debug + Display + Sync {
    fn all_cards(&self) -> &HashMap<u32, &'static ICard>;

    fn all_wild(&self) -> &Vec<&'static ICard>;
    fn all_standard(&self) -> &Vec<&'static ICard>;

    fn wild(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;
    fn standard(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;

    fn from_dbf_id(
        &self,
        id: u32,
    ) -> Result<&'static ICard>;

    fn from_name(
        &self,
        name: &str,
    ) -> Result<&'static ICard>;

    fn hero_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>>;

    fn hero_power_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>>;
}
