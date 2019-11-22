#[derive(Debug)]
pub struct TypeInteraction{
    weaknesses: Vec<String>,
    strengths: Vec<String>,
    ineffectivities: Vec<String>
}

pub enum TypeFactor{
    Weakness(String),
    Strength(String),
    Ineffective(String)
}


impl TypeInteraction{
    
    pub fn new() -> TypeInteraction{
        TypeInteraction{
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            ineffectivities: Vec::new()
        }
    }

    pub fn get_weaknesses(&self) -> Vec<String>{
        self.weaknesses.clone()
    }

    pub fn get_strengths(&self) -> Vec<String> {
        self.strengths.clone()
    }

    pub fn get_ineffectivities(&self) -> Vec<String> {
        self.ineffectivities.clone()
    }

    pub fn add_type_factor(&mut self, factor: TypeFactor){
        match factor {
            TypeFactor::Weakness(t) => self.weaknesses.push(t),
            TypeFactor::Strength(t) => self.strengths.push(t),
            TypeFactor::Ineffective(t) => self.ineffectivities.push(t)
        }
    }
}
