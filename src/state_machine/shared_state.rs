use std::collections::HashSet;
use std::collections::hash_set::Drain;
use std::fmt;

use contracts::entities::playable::IPlayable;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::program::IProgram;
use contracts::state_machine::shared_state::ISharedState;

use enums::EExecutionStates;

// There are 5 registers provided by this state.
const NUM_REGISTERS: u32 = 5;

#[derive(Debug)]
pub struct SharedState<'a> {
    playables: HashSet<&'a IPlayable>,
    card_ids: HashSet<u32>,
    registers: [i32; NUM_REGISTERS as usize],
    flags: u32
}

impl<'a> fmt::Display for SharedState<'a> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "SHARED_STATE [TODO]")
    }
}

impl<'a> SharedState<'a> {
    pub fn new() -> Self {
        Self {
            playables: hashset!{},
            card_ids: hashset!{},
            registers: [0; NUM_REGISTERS as usize],
            flags: 0
        }
    }
}

impl<'a> ISharedState<'a> for SharedState<'a> {
    fn register_num(&self) -> u32 {
        NUM_REGISTERS
    }

    fn playables(&self) -> &HashSet<&IPlayable> {
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
        subj: &'a IPlayable,
    ) {
        self.playables.insert(subj);
    }

    fn add_card_dbf_id(
        &mut self,
        id: u32,
    ) {
        self.card_ids.insert(id);
    }

    fn set_register(
        &mut self,
        register: u32,
        value: i32,
    ) {
        // No-op when out of bounds!
        if register < NUM_REGISTERS {
            self.registers[register as usize] = value;
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

    fn drain_playables(&mut self) -> Drain<&IPlayable> {
        self.playables.drain(..)
    }

    fn drain_card_ids(&mut self) -> Drain<u32> {
        self.card_ids.drain(..)
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
