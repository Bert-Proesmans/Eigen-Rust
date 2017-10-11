#[macro_use]
pub extern crate slog;
extern crate slog_term;
extern crate slog_json;
extern crate slog_async;
extern crate chrono;

#[macro_use]
extern crate eigen_rust;

use std::fs::File;

use chrono::prelude::*;
use slog::Drain;

use eigen_rust::prelude::*;
use eigen_rust::enums::{EFormats,ECardClasses};

fn main() {
    ////////////////////
    // Setup logging. //
    ////////////////////

    // Build logger for the terminal.
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build();

    // Build logger for the terminal
    let file = File::create("app.log").expect("Couldn't open log file");
    let file_drain = slog_json::Json::default(file).fuse();
    let file_drain = slog_async::Async::new(file_drain).build();

    // Build an async combined drain of both drain targets.
    let drain = slog::Duplicate(console_drain, file_drain).fuse();

    // Root logger object; this can be seen as a category for specific log messages.
    // o! macro defines key=>values which will be included in every message.
    let root_logger = slog::Logger::root(drain, o!("version" => "Alpha"));
    // First logged message. Uses the root logger.
    info!(root_logger, "Application started";
        // Additionall context for message.
        "started_at" => format!("{}", Utc::now().to_rfc3339()));


    // Setup game config.
    let mut config = GameConfig::new();
    config.game_format = Some(EFormats::Standard);
    config.player_names[0] = "P1";
    config.player_names[1] = "P2";
    config.player_heroclasses[0] = Some(ECardClasses::Rogue);
    config.player_heroclasses[1] = Some(ECardClasses::Mage);
    // Remove when there are more cards implemented
    config.build_heroes = false;
    config.build_hero_powers = false;

    config.player_decks[0] = Some(vec![
        CARDS.from_name("Lord Jaraxxus").log_unwrap(&root_logger)
    ]);


    info!(root_logger, "GameConfig UPDATED";
                        "config" => format!("{:?}", config));

    // Setup game.
    let mut game = GameManager::new(config, root_logger).log_unwrap(&root_logger);
    game.start();

    let task = EndTurn!(PLAYER_ONE);
    game.process(task);

    // Force an end to the game.
    game.finish();
}
