#[derive(Debug, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum ERarities {
    Common = 1,
    Free = 2,
    Rare = 3,
    Epic = 4,
    Legendary = 5,
    Unknown6 = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum ECardClasses {
    Deathknight = 1,
    Druid = 2,
    Hunter = 3,
    Mage = 4,
    Paladin = 5,
    Priest = 6,
    Rogue = 7,
    Shaman = 8,
    Warlock = 9,
    Warrior = 10,
    Dream = 11,
    Neutral = 12,
}

#[derive(Debug, Primitive)]
#[repr(u32)]
pub enum ERace {
    Bloodelf = 1,
    Draenei = 2,
    Dwarf = 3,
    Gnome = 4,
    Goblin = 5,
    Human = 6,
    Nightelf = 7,
    Orc = 8,
    Tauren = 9,
    Troll = 10,
    Undead = 11,
    Worgen = 12,
    Goblin2 = 13,
    Murloc = 14,
    Demon = 15,
    Scourge = 16,
    Mechanical = 17,
    Elemental = 18,
    Ogre = 19,
    Beast = 20,
    Totem = 21,
    Nerubian = 22,
    Pirate = 23,
    Dragon = 24,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum ECardTypes {
    Invalid = 0, // Needs default value!
    Game = 1,
    Player = 2,
    Hero = 3,
    Minion = 4,
    Spell = 5,
    Enchantment = 6,
    Weapon = 7,
    // TODO; check if this is still used!
    Item = 8,
    // TODO. check if this is still used!
    Token = 9,
    HeroPower = 10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum ECardSets {
    TestTemporary = 1,
    Core = 2,
    Expert1 = 3,
    Hof = 4,
    Missions = 5,
    Demo = 6,
    NoSet = 7,
    Cheat = 8,
    Blank = 9,
    DebugSp = 10,
    Promo = 11,
    Naxx = 12,
    Gvg = 13,
    Brm = 14,
    Tgt = 15,
    Credits = 16,
    HeroSkins = 17,
    Tb = 18,
    Slush = 19,
    Loe = 20,
    Og = 21,
    OgReserve = 22,
    Kara = 23,
    KaraReserve = 24,
    Gangs = 25,
    GangsReserve = 26,
    Ungoro = 27,
    Icecrown = 1001,
}
