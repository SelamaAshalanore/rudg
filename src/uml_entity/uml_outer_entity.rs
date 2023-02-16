#[derive(PartialEq, Debug, Clone)]
pub struct UMLOuterEntity {
    pub name: String,
    pub mod_name: String,
}

impl UMLOuterEntity {
    pub fn new(name: &str, mod_name: &str) -> UMLOuterEntity {
        UMLOuterEntity { name: String::from(name), mod_name: String::from(mod_name) }
    }
}