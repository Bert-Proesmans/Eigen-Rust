use std::fmt;

use contracts::models::IPlayable;
use contracts::state_machine::{IMethod, IProgram, ISharedState};

use core::state_machine::Program;

use enums::contracted::EExecutionStates;

// There are 5 registers provided by this state.
const NUM_REGISTERS: u32 = 5;

#[derive(Debug)]
pub struct SharedState<'state> {
    program_ref: Option<&'state Program<'state>>,
    method_backlog: Vec<Box<IMethod>>,

    playables: Vec<&'state IPlayable>,
    card_ids: Vec<&'static str>,
    registers: [i32; NUM_REGISTERS as usize],
    flags: u32
}

impl<'state> fmt::Display for SharedState<'state> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "SHARED_STATE [TODO]")
    }
}

impl<'state> SharedState<'state> {
    pub fn new() -> Self {
        Self {
            program_ref: None,
            method_backlog: vec![],
            playables: vec![],
            card_ids: vec![],
            registers: [0; NUM_REGISTERS as usize],
            flags: 0
        }
    }

    pub fn for_program(program: &'state Program<'state>) -> Self {
        // Use clean initializer, fill in program reference after.
        let obj = Self::new();
        obj.program_ref = Some(program);
        obj
    }
}

// impl<'program, 'game> ISharedState for
// SharedState<'program, 'game> {
// fn register_num(&self) -> u32 {
// NUM_REGISTERS
// }
//
// fn playables(&self) -> &Vec<&IPlayable> {
// &self.playables
// }
//
// fn card_ids(&self) -> &Vec<&'static str> {
// &self.card_ids
// }
//
// fn flags(&self) -> u32 {
// self.flags
// }
//
// fn add_playable(&mut self, subj: &IPlayable) {
// self.playables.push(subj);
// }
//
// fn add_card_id(&mut self, subj: &'static str) {
// self.card_ids.push(subj);
// }
//
// fn set_register(&mut self, register: u32, value: i32) {
// No-op when out of bounds!
// if register < NUM_REGISTERS {
// self.registers[register as usize] = value;
// }
// }
//
// fn set_flags(&mut self, subj: u32) {
// self.flags = subj;
// }
//
// fn push_method(&mut self, method: Box<IMethod>, fast:
// bool) -> Result<EExecutionStates, bool> {
//
// if let Some(&program_ref) = self.program_ref {
// if fast == true {
// Directly invoke the method within the program.
// return Ok(program_ref.fast_execute(method));
// } else {
// Store the method so program can pull them for proccesing.
// self.method_backlog.push(method);
// }
// }
//
// Ok(EExecutionStates::Ready)
// }
//
// fn clear_all(&mut self) {
// self.drain_playables();
// self.drain_card_ids();
//
// for idx in 0..NUM_REGISTERS {
// self.clear_register(idx);
// }
//
// self.clear_flags();
// }
//
// fn drain_playables(&mut self) ->
// Iterator<Item=&IPlayable> {
// self.playables.drain()
// }
//
// fn drain_card_ids(&mut self) -> Iterator<Item=&'static
// str> {
// self.card_ids.drain()
// }
//
// fn clear_register(&mut self, idx: u32) {
// self.set_register(idx, 0);
// }
//
// fn clear_flags(&mut self) {
// self.flags = 0;
// }
// }
