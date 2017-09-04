// #[macro_export] -> Uncomment when this macro is needed in other crates.
macro_rules! card {
    // This match rule is used to be robust about trailing comma's, since the last field COULD have
    // a trailing comma.
    (id: $id_val:expr, $($field:ident: $value:expr,)+) => { card!(id: $id_val, $($field: $value),+) };

    // Actual implementation of the macro.
    // The macro ENFORCES the first argument to be the ID of the card!
    // The field name equals `id`.
    (id: $id_val:expr, $($field:ident: $value:expr),*) => {
        {
            // Build new card for ID.
            let mut _c = ::core::models::card::Card::new($id_val);
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
