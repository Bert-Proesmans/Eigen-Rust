
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
		pub struct Death {}

		#[derive(Debug)]
		pub struct AuraChange {}

		#[derive(Debug)]
		pub struct EntityChange {}
    }
}

use state_machine::states;

#[derive(Debug)]
enum GameSteps {
    Invalid(GameProcessor<states::Invalid>),
    // Error(GameProcessor<states::Error>),
    
    Start(GameProcessor<states::Start>),
    StartShuffle(GameProcessor<states::StartShuffle>),
    StartDraw(GameProcessor<states::StartDraw>),
    Finished(GameProcessor<states::Finished>),
    
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
    // pub fn step(self) -> Self {
    // 	match self {
    // 	    GameSteps::Start(val) => GameSteps::StartShuffle(val.into()),
    // 	    None => expr,
    // 	}
    // }
}

#[derive(Debug)]
enum ProcessingSteps {
    Trigger(GameProcessor<states::Trigger>),
    Effect(GameProcessor<states::Effect>),
    Death(GameProcessor<states::Death>),

    AuraChange(GameProcessor<states::AuraChange>),
    EntityChange(GameProcessor<states::EntityChange>),
}

#[derive(Debug)]
struct GameProcessor<S> {
    state: S,
    entities: u32,   
    program_state: u32, 
}

impl GameProcessor<states::Invalid> {
    pub fn new() -> Self {
    	Self {
    		state: states::Invalid::new(),
    		entities: 0,
    	}
    }
}

#[derive(Debug)]
struct GameFactory {
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
}
