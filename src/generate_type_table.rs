
use super::poke_type::{TypeInteraction, TypeFactor};
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
struct TypeName{
    name: String 
}

#[derive(Deserialize)]
struct SerializeTypeFactor{
    type_one: String,
    type_two: String,
    factor: String
}

pub fn generate_type_table() -> Option<HashMap<String, TypeInteraction>> {
    let types = include_str!("static/types.csv");
    let type_weaknesses = include_str!("static/weaknesses.csv");
    let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(types.as_bytes());
    let mut poke_types_map: HashMap<String, TypeInteraction> = HashMap::new();
    for record in reader.records(){
        let poke_types_map: &mut HashMap<String, TypeInteraction> = &mut poke_types_map;
        match record {
            Ok(typename) => {
                let typename: TypeName = typename.deserialize(None).unwrap();
                poke_types_map.insert(typename.name.clone(), TypeInteraction::new());
            },
            Err(_) => println!("Couldn't parse type")
        };
    }
    let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(type_weaknesses.as_bytes());
    for record in reader.records(){
        match record{
            Ok(serialized_typefactor) => {
                let poke_types_map: &mut HashMap<String, TypeInteraction> = &mut poke_types_map;
                let serialized_typefactor: SerializeTypeFactor = match serialized_typefactor.deserialize(None) {
                    Ok(t) => t,
                    Err(_) => continue
                };
                let type_one = serialized_typefactor.type_one;
                let type_two = serialized_typefactor.type_two;
                let factor = serialized_typefactor.factor;
                match factor.as_str() {
                    "2" => match poke_types_map.get_mut(&type_one){
                        Some(t) => t.add_type_factor(TypeFactor::Weakness(type_two)),
                        None => {
                            println!("{}", type_one);
                            continue;
                        }
                    },
                    "0.5" => match poke_types_map.get_mut(&type_one){
                        Some(t) => t.add_type_factor(TypeFactor::Strength(type_two)),
                        None => {
                            println!("{}", type_one);
                            continue;
                        }
                    },
                    "0" => match poke_types_map.get_mut(&type_one){
                        Some(t) => t.add_type_factor(TypeFactor::Ineffective(type_two)),
                        None => {
                            println!("{}", type_one);
                            continue;
                        }
                    },
                    _ => continue
                };
            },
            Err(_) => println!("Couldn't serialize typefactor")
        };
    }
    Some(poke_types_map)
}
