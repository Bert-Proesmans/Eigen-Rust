use std::collections::HashSet;
use std::collections::hash_set::Drain;
use std::fmt;

use contracts::entities::playable::IPlayable;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::program::IProgram;
use contracts::state_machine::shared_state::ISharedState;

use enums::EExecutionStates;

// There are 5 registers provided by this state.
// DON'T SUPPOSE THE REGISTER CONTENT WILL SURVIVE MULTIPLE
// IMETHOD RUNS!
// Use the playables and card_ids for persistent storage.
const NUM_REGISTERS: u32 = 6;

pub const REG_EID_ONE: u32 = 0;
pub const REG_EID_TWO: u32 = 1;
pub const REG_EID_THREE: u32 = 2;

pub const REG_MATH_ONE: u32 = 3;
pub const REG_MATH_TWO: u32 = 4;
pub const REG_MATH_THREE: u32 = 5;

pub struct SharedState {
    // The playables are stored by their entity ID.
    // It's not allowed to hold a reference to Playable objects if these
    // objects are contained by the container of this state.
    playables: HashSet<u32>,
    card_ids: HashSet<u32>,
    registers: [i32; NUM_REGISTERS as usize],
    flags: u32
}

impl fmt::Debug for SharedState {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "DBG SHAREDSTATE (TODO)")
    }
}

impl fmt::Display for SharedState {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "SHARED_STATE [TODO]")
    }
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            playables: hashset!{},
            card_ids: hashset!{},
            registers: [0; NUM_REGISTERS as usize],
            flags: 0
        }
    }
}

impl<'px> ISharedState<'px> for SharedState {
    fn register_num(&self) -> u32 {
        NUM_REGISTERS
    }

    fn playables(&self) -> &HashSet<u32> {
        &self.playables
    }

    fn card_ids(&self) -> &HashSet<u32> {
        &self.card_ids
    }

    fn flags(&self) -> u32 {
        self.flags
    }

    fn add_playable(
        &mut self,
        subj: &'px IPlayable<'px>,
    ) {
        self.playables.insert(subj.id());
    }

    fn add_card_dbf_id(
        &mut self,
        id: u32,
    ) {
        self.card_ids.insert(id);
    }

    fn set_register(
        &mut self,
        idx: u32,
        value: i32,
    ) {
        // No-op when out of bounds!
        if idx < NUM_REGISTERS {
            self.registers[idx as usize] = value;
        }
    }

    fn enable_flags(
        &mut self,
        flags: u32,
    ) {
        self.flags |= flags;
    }

    fn disable_flags(
        &mut self,
        flags: u32,
    ) {
        self.flags |= flags;
        self.flags ^= flags;
    }

    fn clear_all(&mut self) {
        self.drain_playables();
        self.drain_card_ids();

        for idx in 0..NUM_REGISTERS {
            self.clear_register(idx);
        }

        self.reset_flags();
    }

    fn drain_playables(&mut self) -> Drain<u32> {
        self.playables.drain()
    }

    fn drain_card_ids(&mut self) -> Drain<u32> {
        self.card_ids.drain()
    }

    fn clear_register(
        &mut self,
        idx: u32,
    ) {
        self.set_register(idx, 0);
    }

    fn reset_flags(&mut self) {
        self.flags = 0;
    }
}
