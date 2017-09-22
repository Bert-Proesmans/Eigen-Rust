use std::collections::HashMap;
use std::fmt;

use contracts::cards::card::ICard;
use enums::ECardClasses;

use super::errors::*;

/// Container object for statically defined card objects
pub trait ICardContainer: fmt::Debug + fmt::Display + Sync {
    /// Returns the set of all registered card objects,
    /// index by their database ID
    fn all_cards(&self) -> &HashMap<u32, &'static ICard>;

    /// Returns a vector of all cards which belong to the
    /// WILD card set
    fn all_wild(&self) -> &Vec<&'static ICard>;

    /// Returns a vector of all cards which belong to the
    /// STANDARD card set
    fn all_standard(&self) -> &Vec<&'static ICard>;

    /// Returns a vector of all cards belonging to the WILD
    /// card set, grouped
    /// by their card class
    fn wild(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;

    /// Returns a vector of all cards belonging to the
    /// STANDARD card set, grouped
    /// by their card class
    fn standard(&self) -> &HashMap<ECardClasses, Vec<&'static ICard>>;

    /// Tries to return the registered card matching the
    /// provided database ID
    fn from_dbf_id(
        &self,
        id: u32,
    ) -> Result<&'static ICard>;

    /// Tries to return the registered card matching the
    /// provided name
    fn from_name(
        &self,
        name: &str,
    ) -> Result<&'static ICard>;

    /// Tries to return all HERO type cards belonging to
    /// the provided card class
    fn hero_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>>;

    /// Tries to return all HERO POWER type cards belonging
    /// to the provided card
    /// class
    fn hero_power_cards(
        &self,
        class: ECardClasses,
    ) -> Result<Vec<&'static ICard>>;
}
