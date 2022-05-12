
#[derive(PartialEq, Debug)]
pub enum UMLClassKind {
    UMLClass,
    UMLTrait,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub struct UMLClass {
    pub name: String,
    method_names: Vec<String>,
    fields: Vec<String>,
    pub kind: UMLClassKind
}


impl UMLClass {
    pub fn new(name: &str, fields: Vec<String>, method_names: Vec<String>, kind: UMLClassKind) -> UMLClass {
        UMLClass { name: String::from(name), fields: fields, method_names: method_names, kind: kind}
    }

    pub fn merge_from(&mut self, from: &mut UMLClass) -> () {
        if self.name != from.name {()}
        self.method_names.append(&mut from.method_names);
    }

    pub fn get_method_names(&self) -> Vec<String> {
        self.method_names.clone()
    }

    pub fn get_field_names(&self) -> Vec<String> {
        self.fields.clone()
    }
}