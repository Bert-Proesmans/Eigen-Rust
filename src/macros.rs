/////////////////
// Define card //
/////////////////

// TODO; Needs a way to pass switches into macro's
// example is disabling validation; card!{@NO_VALIDATE id:""
// ,...}
//
// The macro ENFORCES the first argument to be the ID of
// the card!
// The field name equals `id`.

// #[macro_export] -> Uncomment when this macro is needed
// in other crates.
macro_rules! card {
    // This match rule is used to be robust about trailing comma's, since the last field COULD have
    // a trailing comma.
    (dbf_id: $dbf_id_val:expr, card_id: $card_id_val:expr, $($field:ident: $value:expr,)+)
    => { card!(dbf_id: $dbf_id_val, card_id: $card_id_val, $($field: $value),+) };

    // Actual implementation of the macro.
    // The macro ENFORCES the first argument to be the ID of the card!
    // The field name equals `id`.
    (dbf_id: $dbf_id_val:expr, card_id: $card_id_val:expr, $($field:ident: $value:expr),*)
    => {
            {
                // Build new card for ID.
                let mut _c = $crate::cards::card::Card::new($dbf_id_val, $card_id_val);
                // Set data into card, which es provided to the macro
                // in the form 'field => value'.
                $(
                    _c.$field = $value;
                )*
                // Return the card object, after finalizing it.
                _c.validate().finalize()
            }
    };
}

macro_rules! card_novalidate {
    // This match rule is used to be robust about trailing comma's, since the last field COULD have
    // a trailing comma.
    (dbf_id: $dbf_id_val:expr, card_id: $card_id_val:expr, $($field:ident: $value:expr,)+)
    => { card!(dbf_id: $dbf_id_val, card_id: $card_id_val, $($field: $value),+) };

    // Actual implementation of the macro.
    // The macro ENFORCES the first argument to be the ID of the card!
    // The field name equals `id`.
    (dbf_id: $dbf_id_val:expr, card_id: $card_id_val:expr, $($field:ident: $value:expr),*) => {
        {
            // Build new card for ID.
            let mut _c = $crate::cards::card::Card::new($dbf_id_val, $card_id_val);
            // Set data into card, which es provided to the macro
            // in the form 'field => value'.
            $(
                _c.$field = $value;
            )*
            // Return the card object, after finalizing it.
            _c.finalize()
        }
    };
}

macro_rules! register_result_type {
    (@INNER_CODE, $T_arg:tt) => {
        fn log_unwrap(self, logger: &::slog::Logger) -> $T_arg {
            if let Err(ref e) = self {
                crit!(logger, ""; "error" => %e.display_chain());
                panic!("Critical error, see log");
            }

            self.unwrap()
        }

        fn print_unwrap(self) -> $T_arg {
            if let Err(ref e) = self {
                println!("{:}", e.display_chain());
                panic!("Critical error, see stdout");
            }

            self.unwrap()
        }
    };

    ($error_type:ty) => {
        impl<T> $crate::errors::ResultLogging<T> for ::std::result::Result<T, $error_type>  {
            register_result_type!(@INNER_CODE, T);
        }
    };
}

macro_rules! method_impl_gen {
    // Assign an index to all passed arguments.
    (@count $name:ident; $count:expr,; $($idx:expr => $generic:ident),*) => {
        method_impl_gen!(@build $name; $($idx => $generic),*);
    };

    (@count $name:ident; $count:expr, $head:ident, $($tail:ident),*; $($idx:expr => $generic:ident),*) => {
        method_impl_gen!(@count $name; $count + 1usize, $($tail),*; $($idx => $generic,)* $count => $head);
    };

    // Create implementation.
    (@build $StructName:ident; $($idx:expr => $generic:ident),*) => {
        #[derive(Debug)]
        pub struct $StructName<F, $($generic,)*>
            where
            // P: IProgram<'p> + 'p,
            // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
            F: Fn(&mut IProgram, ($($generic,)*)) -> EExecutionStates + fmt::Debug,
        {
                method: F,
                last_state: EExecutionStates,
                args: ($($generic),*),
                // phantom: PhantomData<&'p P>,
        }

        impl<F, $($generic,)*> $StructName<F, $($generic,)*>
            where
            // P: IProgram<'p> + 'p,
            // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
            F: Fn(&mut IProgram, ($($generic,)*)) -> EExecutionStates + fmt::Debug,
        {
            pub fn construct(method: F, args: ($($generic),*) ) -> Result<Self, String> {
                Ok(Self {
                    method: method,
                    last_state: EExecutionStates::Invalid,
                    args: args,
                    // phantom: PhantomData,
                })
            }
        }

        impl<F, $($generic,)*> IMethod for $StructName<F, $($generic,)*>
            where
            // P: IProgram<'p> + 'p,
            // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
            F: Fn(&mut IProgram, ($($generic,)*)) -> EExecutionStates  + fmt::Debug,
        {
            /// Returns the state value of this method object
            fn state(&self) -> EExecutionStates {
                self.last_state
            }

            /// Run the code held by this method object
            fn run(
                &mut self,
                state: &mut IProgram,
            ) -> EExecutionStates {
                match self.last_state {
                    EExecutionStates::Finished => (),
                    EExecutionStates::Abort => (),
                    _ => self.last_state = (self.method)(state, self.args),
                };

                self.last_state
            }
        }

        impl<F, $($generic,)*> fmt::Display for $StructName<F, $($generic,)*>
            where
            // P: IProgram<'p> + 'p,
            // for <'r> F: Fn(&'r mut P) -> EExecutionStates + fmt::Debug
            F: Fn(&mut IProgram, ($($generic,)*)) -> EExecutionStates + fmt::Debug,
        {
            fn fmt(
                &self,
                f: &mut fmt::Formatter,
            ) -> fmt::Result {
                write!(f, "METHOD [TODO]")
            }
        }
    };

    ($name:ident; $($Arg: ident,)+) => {
        method_impl_gen!($name; $($Arg),+);
    };

    // This includes the empty clause!
    ($name:ident; $($Arg: ident),*) => {
       // Assign an index to each argument.
       // At the end of the process the implementations will be built.
       method_impl_gen!(@count $name; 0usize, $($Arg,)*;);
    };
}
