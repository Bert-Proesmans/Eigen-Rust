use enums::{EFormats, ECardClasses};
use core::Card;

pub const MAX_PLAYERS: usize = 2;
pub const MAX_MINIONS_IN_BOARDZONE: usize = 7;
pub const MAX_ENTITIES_IN_DECKZONE: usize = 120;

#[derive(Debug, Clone)]
pub struct GameConfig {
    game_format: Option<EFormats>,
    starting_player_idx: u32, // Defaults to 1 (player 1)

    /*
     * Player data
     * The amount of players must match MAX_PLAYERS constant.
     */
    player_one_name: Option<&'static str>,
    player_two_name: Option<&'static str>,

    player_one_heroclass: Option<ECardClasses>,
    player_two_heroclass: Option<ECardClasses>,

    player_one_deck: Option<Vec<&'static Card>>,
    player_two_deck: Option<Vec<&'static Card>>,

    fill_decks: bool,
    shuffle: bool,

    max_minions_on_board: usize,
    max_entities_in_deck: usize,
}

#[derive(Debug)]
pub enum GameConfigError {
    NoFormat,
    StartingPlayerOOB, // Out of Bounds!
    NoName(u32),
    NoHeroClass(u32),
    NoDeck(u32),
}

impl GameConfig {
    fn new() -> GameConfig {
        GameConfig {
            game_format: None,
            starting_player_idx: 1,
            player_one_name: Some("Player 1"),
            player_two_name: Some("Player 2"),
            player_one_heroclass: None,
            player_two_heroclass: None,
            player_one_deck: None,
            player_two_deck: None,
            fill_decks: false,
            shuffle: false,
            max_minions_on_board: MAX_MINIONS_IN_BOARDZONE,
            max_entities_in_deck: MAX_ENTITIES_IN_DECKZONE,
        }
    }
}
