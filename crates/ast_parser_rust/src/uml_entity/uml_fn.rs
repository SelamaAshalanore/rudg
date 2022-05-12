
#[derive(PartialEq, Debug)]
pub struct UMLFn {
    pub name: String,
    pub full_name: String
}

impl UMLFn {
    pub fn new(name: &str, full_name: &str) -> UMLFn {
        UMLFn { name: String::from(name), full_name: String::from(full_name) }
    }
}