use game::GameMachine;
use state_machine::{action_states, core_states, trigger_states, wait_states};

use entities::core_entities::{EController, GAME_ENTITY_ID};
use game_tags::EGameTags;

///////////////
// StartGame //
///////////////

#[macro_export]
macro_rules! StartGame {
    () => {
    	|machine| {
    		$crate::operations::play_options::op_start_game(machine, ())
    	}
    }
}

pub fn op_start_game(
    machine: GameMachine<core_states::Wait<wait_states::Start>>,
    args: (),
) -> Result<GameMachine<core_states::Wait<wait_states::Input>>, String> {
    Ok(machine.into())
}


/////////////
// EndTurn //
/////////////

#[macro_export]
macro_rules! EndTurn {
    ($player:ident) => {
    	|machine| {
    		$crate::operations::play_options::op_end_turn(machine, ($player,))
    	}
    }
}

pub fn op_end_turn(
    machine: GameMachine<core_states::Wait<wait_states::Input>>,
    args: (EController,),
) -> Result<GameMachine<core_states::Wait<wait_states::Input>>, String> {
    let (test_current_controller,) = args;
    // TODO; Test if it's currently the turn of provided
    // controller.

    let machine = try!(internal_do_end_turn(machine.into()));
    Ok(machine.into())
}

fn internal_do_end_turn(
    machine: GameMachine<core_states::Action<action_states::EndTurn>>,
) -> Result<GameMachine<core_states::PostAction<action_states::EndTurn>>, String> {
    // Pre-trigger<EndTurn>
    let machine: GameMachine<core_states::Trigger<trigger_states::EndTurn>> = machine.into();
    // TODO
    let machine = try!(machine.signal(GAME_ENTITY_ID, EGameTags::TurnPlayerIdx, 0, 0));
    let machine = try!(machine.death_processing());

    // Pre-Trigger<StartTurn>
    let machine: GameMachine<core_states::Trigger<trigger_states::StartTurn>> = machine.into();
    // TODO
    let machine = try!(machine.signal(GAME_ENTITY_ID, EGameTags::TurnPlayerIdx, 0, 0));
    let machine = try!(machine.death_processing());

    Ok(machine.into())
}


/////////////
// Concede //
/////////////

macro_rules! Concede {
    ($player:ident) => {
    	|machine| {
    		$crate::operations::play_options::op_concede(machine, ($player,))
    	}
    }
}

pub fn op_concede(
    machine: GameMachine<core_states::Wait<wait_states::Input>>,
    args: (EController,),
) -> Result<GameMachine<core_states::Wait<wait_states::Input>>, String> {
    // let (test_current_controller,) = args;
    // TODO; Test if it's currently the turn of provided
    // controller.

    // let machine = try!(internal_do_concede(machine.into()));
    // Ok(machine.into())

    unimplemented!()
}

fn internal_do_concede(
    machine: GameMachine<core_states::Action<action_states::Concede>>,
) -> Result<GameMachine<core_states::PostAction<action_states::Concede>>, String> {
    unimplemented!()
}
