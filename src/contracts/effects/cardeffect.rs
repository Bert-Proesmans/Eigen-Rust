use std::fmt::{Debug, Display};

pub trait IEffect: Debug + Display + Sync {
    fn trigger_requirement(&self) -> i32;
}
