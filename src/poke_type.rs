use std::cmp::PartialEq;

#[derive(Debug)]
pub struct PokemonType<'a>{
    name: &'a str,
    weaknesses: Option<Vec<PokemonType<'a>>>,
    strengths: Option<Vec<PokemonType<'a>>>,
    ineffectivities: Option<Vec<PokemonType<'a>>>
}

impl <'a> PartialEq for PokemonType<'a>{
    
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name
    }

}

impl <'a> PokemonType<'a>{
    
    pub fn new(name: &str) -> PokemonType{
        PokemonType{
            name: name,
            strengths: None,
            weaknesses: None,
            ineffectivities: None 
        }
    }

    pub fn add_weakness(self, weakness: PokemonType<'_>) -> bool {
        match self.weaknesses{
            Some(mut weaknesses) => {
                if weaknesses.contains(&weakness){
                    return false
                };
                weaknesses.push(weakness);
                true
            },
            None => {
                let mut weaknesses: Vec<PokemonType> = Vec::new();
                weaknesses.push(weakness);
                true
            }
        }
    }
}
