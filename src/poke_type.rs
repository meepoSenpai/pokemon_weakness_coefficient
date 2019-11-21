use std::cmp::PartialEq;

#[derive(Debug)]
pub struct PokemonType<'a> {
    pub name: &'a str,
    weaknesses: Vec<PokemonType<'a>>,
    strengths: Vec<PokemonType<'a>>,
    ineffectivities: Vec<PokemonType<'a>>
}

pub enum TypeFactor<'a> {
    Weakness(PokemonType<'a>),
    Strength(PokemonType<'a>),
    Ineffective(PokemonType<'a>)
}

impl <'a> PartialEq for PokemonType<'a> {
    
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name
    }

}

impl <'a> PokemonType<'a>{
    
    pub fn new(name: &str) -> PokemonType {
        PokemonType{
            name: name,
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            ineffectivities: Vec::new()
        }
    }

    pub fn get_weaknesses(&self) -> &Vec<PokemonType<'a>>{
        &self.weaknesses
    }

    pub fn get_strengths(&self) -> &Vec<PokemonType<'a>> {
        &self.strengths
    }

    pub fn get_ineffectivities(&self) -> &Vec<PokemonType<'a>> {
        &self.ineffectivities
    }

    pub fn add_type_factor(&mut self, factor: TypeFactor<'a>){
        match factor {
            TypeFactor::Weakness(t) => self.weaknesses.push(t),
            TypeFactor::Strength(t) => self.strengths.push(t),
            TypeFactor::Ineffective(t) => self.ineffectivities.push(t)
        }
    }
}
