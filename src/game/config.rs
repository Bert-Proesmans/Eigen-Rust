use contracts::cards::card::ICard;

use enums::{ECardClasses, EFormats};

use super::errors::*;

pub const MAX_PLAYERS: u32 = 2;
pub const MAX_MINIONS_IN_BOARDZONE: usize = 7;
pub const MAX_ENTITIES_IN_DECKZONE: usize = 120;

// Make sure this struct always AUTO-DERIVES CLONE!
#[derive(Debug, Default)]
pub struct GameConfig {
    pub game_format: Option<EFormats>,
    pub starting_player_idx: u32, // Defaults to 1 (player 1)

    // Player data
    // The amount of players must match MAX_PLAYERS constant.
    //
    pub player_names: [&'static str; MAX_PLAYERS as usize],

    pub player_heroclasses: [Option<ECardClasses>; MAX_PLAYERS as usize],

    // It's not possible to have an array of vectors.. or an array of a large amount of cards.
    // This is a serious limitation so we have to fall back to a vector of vectors..
    pub player_decks: Vec<Option<Vec<&'static ICard>>>,

    pub fill_decks: bool,
    pub shuffle: bool,

    pub build_heroes: bool,
    pub build_hero_powers: bool,

    pub max_minions_on_board: usize,
    pub max_entities_in_deck: usize
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            game_format: None,
            starting_player_idx: 1,
            player_names: ["Player 1", "Player 2"],
            player_decks: vec![None, None],
            build_heroes: true,
            build_hero_powers: true,
            max_minions_on_board: MAX_MINIONS_IN_BOARDZONE,
            max_entities_in_deck: MAX_ENTITIES_IN_DECKZONE,
            ..Default::default()
        }
    }

    pub fn validate(self) -> Result<Self> {
        try!(self.game_format.as_ref().ok_or(Error::from_kind(ErrorKind::NoFormat)));

        if self.starting_player_idx < 1 || self.starting_player_idx > MAX_PLAYERS {
            bail!(ErrorKind::StartingPlayerOOB);
        }

        for idx in 0..MAX_PLAYERS {
            try!(self.player_heroclasses[idx as usize].ok_or(Error::from_kind(ErrorKind::NoHeroClass(idx))));

            // Decks are allowed to be partial/empty or non given.
        }

        Ok(self)
    }
}
