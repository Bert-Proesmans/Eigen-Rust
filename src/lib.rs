#[macro_use]
extern crate maplit;

use std::collections::{HashSet,HashMap};
use std::fmt;

pub mod operations {
	pub mod play_options {
		fn end_turn(state: GameProcessor<states::TurnWait>, variables: (u32,)) ->GameProcessor<states:: {

		}
	}
}


pub mod state_machine {

    pub mod states {
    	#[derive(Debug)]
		pub struct Invalid {}

		impl Invalid {
		    pub fn new() -> Self {
		    	Self {}
		    }
		}

		#[derive(Debug)]
		pub struct Error {}

		#[derive(Debug)]
		pub struct Start {}

		#[derive(Debug)]
		pub struct StartShuffle {}

		#[derive(Debug)]
		pub struct StartDraw {}

		#[derive(Debug)]
		pub struct Finished {}

		#[derive(Debug)]
		pub struct Mulligan {}

		#[derive(Debug)]
		pub struct MulliganWait {}

		#[derive(Debug)]
		pub struct MainStart {}

		#[derive(Debug)]
		pub struct MainEnd {}

		#[derive(Debug)]
		pub struct TurnStart {}

		#[derive(Debug)]
		pub struct TurnResource {}

		#[derive(Debug)]
		pub struct TurnDraw {}

		#[derive(Debug)]
		pub struct TurnWait {}

		#[derive(Debug)]
		pub struct TurnEnd {}

		#[derive(Debug)]
		pub struct Play {}

		#[derive(Debug)]
		pub struct Surrender {}

		#[derive(Debug)]
		pub struct Trigger {}

		#[derive(Debug)]
		pub struct Effect {}

		#[derive(Debug)]
		pub struct ImmediateEffect {}

		#[derive(Debug)]
		pub struct Death {}

		#[derive(Debug)]
		pub struct AuraChange {}

		#[derive(Debug)]
		pub struct EntityChange {}
    }
}

use state_machine::states;

#[derive(Debug)]
pub enum GameSteps {
    Invalid(GameProcessor<states::Invalid>),
    // Error(GameProcessor<states::Error>),
    
    Finished(GameProcessor<states::Finished>),
    Start(GameProcessor<states::Start>),
    StartShuffle(GameProcessor<states::StartShuffle>),
    StartDraw(GameProcessor<states::StartDraw>),
    
    Mulligan(GameProcessor<states::Mulligan>),
    MulliganWait(GameProcessor<states::MulliganWait>),

    MainStart(GameProcessor<states::MainStart>),
    MainEnd(GameProcessor<states::MainEnd>),

    TurnStart(GameProcessor<states::TurnStart>),
    TurnResource(GameProcessor<states::TurnResource>),
    TurnDraw(GameProcessor<states::TurnDraw>),
    TurnWait(GameProcessor<states::TurnWait>),
    TurnEnd(GameProcessor<states::TurnEnd>),
    Play(GameProcessor<states::Play>),
    Surrender(GameProcessor<states::Surrender>),

    Processing(ProcessingSteps),
}

impl GameSteps {
    pub fn step(self) -> Self {
    	match self {
    		GameSteps::Invalid(val) => GameSteps::Start(val.into()),
    	    // GameSteps::Start(val) => GameSteps::StartShuffle(val.into()),
    	    _ => unimplemented!{},
    	}
    }
}

#[derive(Debug)]
pub enum ProcessingSteps {
    Trigger(GameProcessor<states::Trigger>),
    ImmediateEffect(GameProcessor<states::ImmediateEffect>),
    Effect(GameProcessor<states::Effect>),
    Death(GameProcessor<states::Death>),

    AuraChange(GameProcessor<states::AuraChange>),
    EntityChange(GameProcessor<states::EntityChange>),
}

const NUM_REGISTERS: usize = 6;

#[derive(Debug)]
struct SharedData {
    playables: HashSet<u32>,
    card_ids: HashSet<u32>,
    registers: [i32; NUM_REGISTERS],
    flags: u32,
}

impl SharedData {
    pub fn new() -> Self {
    	Self {
    		playables: hashset!{},
    		card_ids: hashset!{},
    		registers: [0; NUM_REGISTERS],
    		flags: 0,
    	}
    }
}

pub trait IEntity: fmt::Debug {
    // add code here
}

#[derive(Debug)]
pub struct GameProcessor<S> {
    state: S,
    entities: Vec<Box<IEntity>>,   
    program_state: SharedData, 
}

impl GameProcessor<states::Invalid> {
    pub fn new() -> Self {
    	Self {
    		state: states::Invalid::new(),
    		entities: vec![],
    		program_state: SharedData::new(),
    	}
    }
}

impl From<GameProcessor<states::Invalid>> for GameProcessor<states::Start> {
    fn from(val: GameProcessor<states::Invalid>) -> GameProcessor<states::Start> {
    	GameProcessor {
    		entities: val.entities,
    		program_state: val.program_state,
    		state: states::Start{},
    	}
    }
}

#[derive(Debug)]
pub struct GameFactory {
    config: u32,
    processor: GameSteps,
}

impl GameFactory {
    pub fn new(config: u32) -> Self {
    	Self {
    		config: config,
    		processor: GameSteps::Invalid(GameProcessor::new()),
    	}
    }

    pub fn start_game(self) -> GameSteps {
    	let invalid_state = self.processor;
    	return invalid_state.step();
    }
}
