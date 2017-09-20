use std::fmt;

use contracts::entities::entity::IEntity;

pub mod errors {
    use enums::ECardTypes;

    error_chain!{
        errors {
            NoCastProvided {
                display("The requested type did not provide a cast method")
            }

            ErasedType {
                display("The underlying type is unknown")
            }

            NonMatchingType(expected: ECardTypes, found: ECardTypes) {
                display("The underlying type `{:?}` did not match the requested type `{:?}`", found, expected)
            }
        }
    }
}

use self::errors::*;

// Kinda similar to the nightly trait TryFrom.
// This trait is used to downcast IEntities into their
// struct type.
pub trait IEntityCastable: fmt::Debug + fmt::Display {
    fn try_into<'e>(e: &'e IEntity) -> Result<&'e Self>;
    fn try_into_mut<'e>(e: &'e mut IEntity) -> Result<&'e mut Self>;
}
