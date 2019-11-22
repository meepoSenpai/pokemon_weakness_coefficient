#[macro_use]
extern crate lazy_static;

pub mod poke_type;
mod generate_type_table;

use poke_type::TypeInteraction;
use generate_type_table::generate_type_table;

use std::collections::HashMap;

lazy_static! {
    static ref TYPETABLE: HashMap<String, TypeInteraction> = {
        match generate_type_table() {
            Some(t) => t,
            None => panic!()
        }
    };
}

pub fn recieve_type_table<'a>() -> &'a HashMap<String, TypeInteraction> {
    &TYPETABLE
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
