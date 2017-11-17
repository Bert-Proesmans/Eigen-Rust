use num_traits::ToPrimitive;
use std::result;

// use entities::core_entities::GAME_ENTITY_ID;
use game::GameProcessor;
use state_machine::core_states;
use state_machine::trigger_states;

type Result<A> = result::Result<A, GameProcessor<core_states::Finished>>;

#[derive(Debug, PartialEq, Eq, Primitive)]
pub enum EController {
    ControllerOne = 1,
    ControllerTwo = 2
}

pub const PLAYER_ONE: EController = EController::ControllerOne;
pub const PLAYER_TWO: EController = EController::ControllerTwo;

pub fn end_turn(
    machine: GameProcessor<core_states::Input>,
    variables: (EController,),
) -> Result<GameProcessor<core_states::Input>> {
    // 'machine' is the container of the processing state!
    let (next_player,) = variables;
    let _next_player = next_player.to_u32().expect("Invalid EController value");

    // {
    // let game_entity =
    // machine.get_entity_mut(GAME_ENTITY_ID).expect("No game
    // entity");
    // let current_player =
    // game_entity.tag_value(EGameTags::TurnPlayerIdx);

    //     if next_player != current_player {
    // game_entity.set_tag_value(EGameTags::
    // TurnPlayerIdx, next_player);
    //     }
    // }

    // Phase into endturn.
    let machine: GameProcessor<core_states::Trigger<trigger_states::EndTurn>> = machine.into();
    let machine = try!(handle_end_turn_triggers(machine));

    return Ok(machine.into());
}

pub fn handle_end_turn_triggers(
    _machine: GameProcessor<core_states::Trigger<trigger_states::EndTurn>>,
) -> Result<GameProcessor<core_states::Neutral>> {
    unimplemented!()
}

#[macro_export]
macro_rules! EndTurn {
    ($player:ident) => {
        |machine| {
            $crate::operations::play_options::end_turn(machine, ($player,))
        }
    }
}
