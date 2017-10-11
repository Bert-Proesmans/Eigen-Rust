use std::fmt;
use std::marker::PhantomData;

use contracts::state_machine::program::IProgram;
use contracts::state_machine::method::IMethod;

use enums::EExecutionStates;

method_impl_gen!{ Method; }
method_impl_gen!{ MethodOne; A}

// ////////////////////
// // No arg version //
// ////////////////////

// // #[derive(Debug)]
// pub struct Method<F>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
//     F: Fn(&mut IProgram) -> EExecutionStates + fmt::Debug,
// {
//         method: F,
//         last_state: EExecutionStates,
//         // phantom: PhantomData<&'p P>,
// }

// impl<F> Method<F>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
//     F: Fn(&mut IProgram) -> EExecutionStates + fmt::Debug,
// {
//     pub fn construct(method: F) -> Result<Self, String> {
//         Ok(Self {
//             method: method,
//             last_state: EExecutionStates::Invalid,
//             // phantom: PhantomData,
//         })
//     }
// }

// impl<F> IMethod for Method<F>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
//     F: Fn(&mut IProgram) -> EExecutionStates + fmt::Debug,
// {
//     /// Returns the state value of this method object
//     fn state(&self) -> EExecutionStates {
//         self.last_state
//     }

//     /// Run the code held by this method object
//     fn run(
//         &mut self,
//         state: &mut IProgram,
//     ) -> EExecutionStates {
//         match self.last_state {
//             EExecutionStates::Finished => (),
//             EExecutionStates::Abort => (),
//             _ => self.last_state = (self.method)(state),
//         };

//         self.last_state
//     }
// }

// impl<F> fmt::Display for Method<F>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
//     F: Fn(&mut IProgram) -> EExecutionStates + fmt::Debug,
// {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(f, "METHOD [TODO]")
//     }
// }

// impl<F> fmt::Debug for Method<F>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
//     F: Fn(&mut IProgram) -> EExecutionStates + fmt::Debug,
// {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(f, "METHOD [TODO]")
//     }
// }

// /////////////////////
// // One arg version //
// /////////////////////

// // #[derive(Debug)]
// pub struct MethodOne<F, A>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P, A) -> EExecutionStates + fmt::Debug,
//     F: Fn(&mut IProgram, A) -> EExecutionStates + fmt::Debug,
//     A: Copy + fmt::Debug,
// {
//     method: F,
//     last_state: EExecutionStates,
//     arg_one: A,
//     // phantom: PhantomData<&'p P>,
// }

// impl<F, A> IMethod for MethodOne<F, A>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P, A) -> EExecutionStates + fmt::Debug,
//     F: Fn(&mut IProgram, A) -> EExecutionStates + fmt::Debug,
//     A: Copy + fmt::Debug,
// {
//     fn state(&self) -> EExecutionStates {
//         self.last_state
//     }

//     fn run(&mut self, state: &mut IProgram) -> EExecutionStates {
//         match self.last_state {
//             EExecutionStates::Finished => (),
//             EExecutionStates::Abort => (),
//             _ => self.last_state = (self.method)(state, self.arg_one),
//         };
//         self.last_state
//     }
// }

// impl<F, A> MethodOne<F, A>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P, A) -> EExecutionStates + fmt::Debug,
//     F: Fn(&mut IProgram, A) -> EExecutionStates + fmt::Debug,
//     A: Copy + fmt::Debug,
// {
//     pub fn construct(method: F, arg_one: A) -> Result<Self, String> {
//         Ok(Self {
//             method: method,
//             last_state: EExecutionStates::Invalid,
//             arg_one: arg_one,
//             // phantom: PhantomData,
//         })
//     }
// }

// impl<F,A> fmt::Display for MethodOne<F, A>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P, A) -> EExecutionStates + fmt::Debug,
//     F: Fn(&mut IProgram, A) -> EExecutionStates + fmt::Debug,
//     A: Copy + fmt::Debug,
// {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(f, "METHOD [TODO]")
//     }
// }

// impl<F,A> fmt::Debug for MethodOne<F,A>
//     where
//     // P: IProgram<'p> + 'p,
//     // for <'r> F: Fn(&'r mut P, A) -> EExecutionStates + fmt::Debug,
//     F: Fn(&mut IProgram, A) -> EExecutionStates + fmt::Debug,
//     A: Copy + fmt::Debug,
// {
//     fn fmt(
//         &self,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         write!(f, "METHOD [TODO]")
//     }
// }
