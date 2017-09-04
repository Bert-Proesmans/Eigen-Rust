mod internal;
mod card;
mod game;

//////////////////////////////////////////////////////////////
// The default underlying type for all enumerations is u32! //
//////////////////////////////////////////////////////////////


// Re-export all enums from child modules through this
// module.
pub use self::internal::*;
pub use self::card::*;
pub use self::game::*;

//////////////////////////////////////////////
// Defaults for enums (only when necessary) //
//////////////////////////////////////////////

impl Default for ECardTypes {
    fn default() -> Self {
        ECardTypes::Invalid
    }
}

impl Default for ECardSets {
    fn default() -> Self {
        ECardSets::Invalid
    }
}

/////////////////////////////
// Enum conversion methods //
/////////////////////////////

impl ERarities {
    pub fn from(p: u32) -> Option<Self> {
        // Keep re-using variable `p` to prevent unused variable warnings!
        match p {
            p if p == ERarities::Common as u32 => Some(ERarities::Common),
            p if p == ERarities::Rare as u32 => Some(ERarities::Rare),
            p if p == ERarities::Epic as u32 => Some(ERarities::Epic),
            p if p == ERarities::Legendary as u32 => Some(ERarities::Legendary),
            p if p == ERarities::Free as u32 => Some(ERarities::Free),
            p if p == ERarities::Unknown6 as u32 => Some(ERarities::Unknown6),
            _ => None,
        }
    }
}

impl ECardClasses {
    pub fn from(p: u32) -> Option<Self> {
        match p {
            p if p == ECardClasses::Deathknight as u32 => Some(ECardClasses::Deathknight),
            p if p == ECardClasses::Druid as u32 => Some(ECardClasses::Druid),
            p if p == ECardClasses::Hunter as u32 => Some(ECardClasses::Hunter),
            p if p == ECardClasses::Mage as u32 => Some(ECardClasses::Mage),
            p if p == ECardClasses::Paladin as u32 => Some(ECardClasses::Paladin),
            p if p == ECardClasses::Priest as u32 => Some(ECardClasses::Priest),
            p if p == ECardClasses::Rogue as u32 => Some(ECardClasses::Rogue),
            p if p == ECardClasses::Shaman as u32 => Some(ECardClasses::Shaman),
            p if p == ECardClasses::Warlock as u32 => Some(ECardClasses::Warlock),
            p if p == ECardClasses::Warrior as u32 => Some(ECardClasses::Warrior),
            p if p == ECardClasses::Neutral as u32 => Some(ECardClasses::Neutral),
            // Dream class not implemented because it doesn't seem to be used.
            // TODO; remove ECardClasses::Dream from enum
            _ => None,
        }
    }
}

impl ECardTypes {
    pub fn from(p: u32) -> Option<Self> {
        match p {
            p if p == ECardTypes::Game as u32 => Some(ECardTypes::Game),
            p if p == ECardTypes::Player as u32 => Some(ECardTypes::Player),
            p if p == ECardTypes::Hero as u32 => Some(ECardTypes::Hero),
            p if p == ECardTypes::Minion as u32 => Some(ECardTypes::Minion),
            p if p == ECardTypes::Spell as u32 => Some(ECardTypes::Spell),
            p if p == ECardTypes::Enchantment as u32 => Some(ECardTypes::Enchantment),
            p if p == ECardTypes::Weapon as u32 => Some(ECardTypes::Weapon),
            p if p == ECardTypes::Item as u32 => Some(ECardTypes::Item),
            p if p == ECardTypes::Token as u32 => Some(ECardTypes::Token),
            p if p == ECardTypes::HeroPower as u32 => Some(ECardTypes::HeroPower),
            _ => None,
        }
    }
}

impl ECardSets {
    pub fn from(p: u32) -> Option<ECardSets> {
        match p {
            // We only consider the major sets.
            // Af a card belongs to the other sets (barely used), we return a None.
            p if p == ECardSets::Core as u32 => Some(ECardSets::Core),
            p if p == ECardSets::Expert1 as u32 => Some(ECardSets::Expert1),
            p if p == ECardSets::Hof as u32 => Some(ECardSets::Hof),
            p if p == ECardSets::Naxx as u32 => Some(ECardSets::Naxx),
            p if p == ECardSets::Gvg as u32 => Some(ECardSets::Gvg),
            p if p == ECardSets::Brm as u32 => Some(ECardSets::Brm),
            p if p == ECardSets::Tgt as u32 => Some(ECardSets::Tgt),
            p if p == ECardSets::Loe as u32 => Some(ECardSets::Loe),
            p if p == ECardSets::Og as u32 => Some(ECardSets::Og),
            p if p == ECardSets::Kara as u32 => Some(ECardSets::Kara),
            p if p == ECardSets::Gangs as u32 => Some(ECardSets::Gangs),
            p if p == ECardSets::Ungoro as u32 => Some(ECardSets::Ungoro),
            p if p == ECardSets::Icecrown as u32 => Some(ECardSets::Icecrown),
            _ => None,
        }
    }
}
