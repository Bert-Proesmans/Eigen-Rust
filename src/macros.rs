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
                crit!(logger, ""; "error" => %e);
                panic!("Critical error, see log");
            }

            self.unwrap()
        }
    };

    ($error_type:ty) => {
        impl<T> $crate::errors::ErrorLogging<T> for ::std::result::Result<T, $error_type>  {
            register_result_type!(@INNER_CODE, T);
        }
    };
}
