struct KBClass {
    name: String,
    description: String,
    parametres: Vec<Parameter>
}


impl KBClass {
    pub fn new(name: &str, description: &str) -> KBClass {
        KBClass {
            name: name.to_string(),
            description: description.to_string(),
            parametres: vec![],
        }
    }

    pub fn add_parameter(mut self, name: &str, description: &str) -> KBClass {
        self.parametres.push(
            Parameter {
                name: name.to_string(),
                description: description.to_string(),
            }
        );
        self
    }
}


struct Parameter {
    name: String,
    description: String,
}

impl Parameter {
    pub fn new(name: &str, description: &str) -> Parameter {
        Parameter {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests;
