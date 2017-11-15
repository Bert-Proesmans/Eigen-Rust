use std::collections::HashSet;

const NUM_REGISTERS: usize = 6;

#[derive(Debug)]
pub(crate) struct SharedData {
    pub(crate) playables: HashSet<u32>,
    pub(crate) card_ids: HashSet<u32>,
    pub(crate) registers: [i32; NUM_REGISTERS],
    pub(crate) flags: u32,
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
