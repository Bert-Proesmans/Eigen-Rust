use num_traits::{FromPrimitive, ToPrimitive};

use game::GameMachine;
use state_machine::{core_states, wait_states};
use state_machine::action_trigger_states as ats;
use state_machine::dynamic::DynamicOperations as op;

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
    machine: GameMachine<core_states::Action<ats::EndTurn>>,
) -> Result<GameMachine<core_states::PostAction<ats::EndTurn>>, String> {
    // Pre-trigger<EndTurn>
    let machine: GameMachine<core_states::Effect<ats::EndTurn>> = machine.into();

    let current_player_eid = machine.get_current_player_eid();
    let remaining_turns = machine.get_entity(current_player_eid).unwrap().tag_value(EGameTags::RemainingTurns);

    // TODO
    let machine = machine.signal(current_player_eid, EGameTags::RemainingTurns, op::DecreaseOne);
    let machine = try!(machine.update());
    let machine = try!(machine.death_processing());

    // Starting next turn SHOULD BE AUTOMATIC!

    // let current_player_idx = machine.get_entity(GAME_ENTITY_ID).unwrap().tag_value(EGameTags::TurnPlayerIdx);
    // let current_player_idx = EController::from_u32(current_player_idx).unwrap();

    // // Only swap current player if the number of remaining turns hit 0
    // let next_player_idx = current_player_idx; // Implicit copy
    // if (remaining_turns - 1) == 0 {
    //     let next_player_idx = next_player_idx.next();
    // }

    // let current_player_idx = current_player_idx.to_u32().unwrap();
    // let next_player_idx = next_player_idx.to_u32().unwrap();

    // // Pre-Trigger<StartTurn>
    // let machine: GameMachine<core_states::Trigger<ats::StartTurn>> = machine.into();
    // // TODO
    // let machine = try!(machine.signal(GAME_ENTITY_ID, EGameTags::TurnPlayerIdx, current_player_idx, next_player_idx));
    // let machine = try!(machine.death_processing());

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
    machine: GameMachine<core_states::Action<ats::Concede>>,
) -> Result<GameMachine<core_states::PostAction<ats::Concede>>, String> {

    unimplemented!()
}
