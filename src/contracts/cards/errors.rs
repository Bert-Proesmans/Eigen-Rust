use enums::ECardClasses;

error_chain!{
    errors {
        UnknownDBFID(id: u32) {
            display("The DBF id `{}` is unknown", id)
        }

        UnknownCardID(id: String) {
            display("The Card id `{}` is unknown", id)
        }

        UnknownName(name: String) {
            display("The card name `{}` is unknown", name)
        }

        EmptyClassMatch(class: ECardClasses) {
            display("No matching cards found for class {:?}", class)
        }
    }
}
