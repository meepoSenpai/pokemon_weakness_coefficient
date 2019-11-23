#[macro_use]
extern crate lazy_static;

mod poke_type;
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

pub fn calculate_weaknesses(team_types: Vec<String>) -> Result<Vec<String>, String> {
    if team_types.len() < 1 && team_types.len() > 12{
       return Err(String::from("Your team consists of either too many or too few different types"));
    }
    let mut all_weaknesses: Vec<String> = Vec::new();
    let mut all_strengths: Vec<String> = Vec::new();
    for pokemon_type in team_types {
        match TYPETABLE.get(&pokemon_type) {
            Some(type_interaction) => {
                all_weaknesses.append(&mut type_interaction.get_weaknesses());
                all_strengths.append(&mut type_interaction.get_strengths());
            },
            None => continue
        };
    }
    let mut unblocked_weaknesses: Vec<String> = Vec::new();
    for pokemon_type in all_weaknesses {
        if all_strengths.contains(&pokemon_type) {
            continue;
        }
        unblocked_weaknesses.push(pokemon_type);
        
    }
    Ok(unblocked_weaknesses)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
