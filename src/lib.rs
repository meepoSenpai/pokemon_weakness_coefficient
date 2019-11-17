pub mod poke_type;

#[cfg(test)]
mod tests {
    use super::poke_type::PokemonType;

    #[test]
    fn it_works() {
        let mut sometype = PokemonType::new("Dank");
        match sometype.weaknesses {
            Some(mut weak_list) => weak_list.push(PokemonType::new("Water")),
            None => println!("Nothing found")
        };
        assert_eq!(sometype.name, "Dank");
    }
}
