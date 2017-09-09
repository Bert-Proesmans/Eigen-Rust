// #[macro_export] -> Uncomment when this macro is needed
// in other crates.
macro_rules! cast_entity {
    ($entity_var:ident, ($($required_tag:tt)*), $target_type:ty) => {
    {
        let _any_type: &::std::any::Any = try!(
            match $entity_var.reference_card().card_type() {
                Some($($required_tag)*) => Ok($entity_var.as_any()),
                Some(value) => Err(::enums::contracted::EntityCastError::NonMatchingType {
                    found: value,
                    expected: $($required_tag)*,
                }),
                _ => Err(::enums::contracted::EntityCastError::ErasedType),
            }
        );

        Ok(_any_type.downcast_ref::<$target_type>().expect("Downcasting IEntity faild"))
    }
    };
}

// #[macro_export] -> Uncomment when this macro is needed
// in other crates.
macro_rules! cast_entity_mut {
    ($entity_var:ident, ($($required_tag:tt)*), $target_type:ty) => {
    {
        let mut _any_type: &mut ::std::any::Any = try!(
            match $entity_var.reference_card().card_type() {
                Some($($required_tag)*) => Ok($entity_var.as_any_mut()),
                Some(value) => Err(::enums::contracted::EntityCastError::NonMatchingType {
                    found: value,
                    expected: $($required_tag)*,
                }),
                _ => Err(::enums::contracted::EntityCastError::ErasedType),
            }
        );

        Ok(_any_type.downcast_mut::<$target_type>().expect("Downcasting IEntity faild"))
    }
    };
}
