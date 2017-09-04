use std::fmt;

use contracts::effects::IEffect;

#[derive(Debug)]
pub struct CardEffect {}

impl fmt::Display for CardEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "EFFECT [TODO]")
    }
}

impl IEffect for CardEffect {
    fn trigger_requirement(&self) -> i32 {
        0
    }
}
