pub mod poke_type;

#[cfg(test)]
mod tests {
    use super::poke_type::PokemonType;
    use super::poke_type::TypeFactor;

    #[test]
    fn it_works() {
        let mut sometype = PokemonType::new("Dank");
        sometype.add_type_factor(TypeFactor::Weakness(PokemonType::new("Not So Dank")));
        let name = sometype.get_weaknesses().first().unwrap().name;
        assert_eq!(name, "Not So Dank");
    }
}
