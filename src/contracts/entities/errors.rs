error_chain!{

    errors {
        UnknownEntityID(id: u32) {
            display("The requested entity ID wasn't found")
        }

        MissingEntity {
            display("The operation couldn't succeed because of a missing entity")
        }
    }

    links {
        InvalidEntityData(::contracts::entities::entity_data::errors::Error,
                            ::contracts::entities::entity_data::errors::ErrorKind);
        InvalidCast(::contracts::entities::entity_castable::errors::Error,
                        ::contracts::entities::entity_castable::errors::ErrorKind);
        InvalidCardData(::contracts::cards::errors::Error
                        ,::contracts::cards::errors::ErrorKind);
    }
}
