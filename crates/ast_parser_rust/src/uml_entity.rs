use ra_ap_syntax::{ast::{self, HasModuleItem}, SourceFile};

use crate::ast_parser::{HasUMLFn, HasUMLClass, HasUMLDependency, HasUMLRelation};

pub enum UMLRelationKind {
    UMLAggregation,
    UMLComposition,
    UMLDependency
}

pub struct UMLRelation {
    pub from: String,
    pub to: String,
    pub kind: UMLRelationKind
}

impl UMLRelation {
    pub fn new(from: &str, to: &str, kind: UMLRelationKind) -> UMLRelation {
        UMLRelation { from: String::from(from), to: String::from(to), kind: kind }
    }
}

pub struct UMLAggregation {
    pub from: String,
    pub to: String
}


impl UMLAggregation {
    pub fn new(from: &str, to: &str) -> UMLAggregation {
        UMLAggregation { from: String::from(from), to: String::from(to) }
    }
}

pub struct UMLDependency {
    pub from: String,
    pub to: String
}


impl UMLDependency {
    pub fn new(from: &str, to: &str) -> UMLDependency {
        UMLDependency { from: String::from(from), to: String::from(to) }
    }
}

pub struct UMLComposition {
    pub from: String,
    pub to: String
}

impl UMLComposition {
    pub fn new(from: &str, to: &str) -> UMLComposition {
        UMLComposition { from: String::from(from), to: String::from(to) }
    }
}


pub struct UMLFn {
    pub name: String,
    pub dependent_fn_names: Vec<String>,
    pub full_name: String
}

impl UMLFn {
    pub fn new(name: &str, dependent_fn_names: Vec<String>, full_name: &str) -> UMLFn {
        UMLFn { name: String::from(name), dependent_fn_names: dependent_fn_names, full_name: String::from(full_name) }
    }
}

pub struct UMLClass {
    pub name: String,
    method_names: Vec<String>,
    fields: Vec<String>,
}


impl UMLClass {
    pub fn new(name: &str, fields: Vec<String>, method_names: Vec<String>) -> UMLClass {
        UMLClass { name: String::from(name), fields: fields, method_names: method_names}
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

pub struct UMLModule {
    pub structs: Vec<(String, UMLClass)>,
    pub fns: Vec<UMLFn>,
    pub dependency: Vec<UMLDependency>,
    pub relations: Vec<UMLRelation>
}

impl UMLModule {
    pub fn new() -> UMLModule {
        UMLModule { structs: vec![], fns: vec![], dependency: vec![], relations: vec![]}
    }

    pub fn parse_source_file(&mut self, src_file: SourceFile) -> () {
        // visit all items in SourceFile and extract dot entities from every type of them
        for item in src_file.items() {
            match item {
                ast::Item::Fn(f) => {
                    self.fns.append(&mut f.get_uml_fn());
                    self.relations.append(&mut f.get_uml_relations());
                },
                ast::Item::Impl(ip) => {
                    self.add_structs(ip.get_uml_class());
                    self.dependency.append(&mut ip.get_uml_dependency());
                    self.relations.append(&mut ip.get_uml_relations());
                },
                ast::Item::Struct(st) => {
                    self.add_structs(st.get_uml_class());
                    self.relations.append(&mut st.get_uml_relations());
                },
                _ => (),
            }
        }
    }

    fn add_structs(&mut self, st_list: Vec<UMLClass>) -> () {
        for mut st in st_list {
            if self.get_struct_names().contains(&st.name) {
                println!("{} struct exists!", &st.name);
                self.get_mut_struct(&st.name).unwrap().merge_from(&mut st);
            } else {
                let st_name = st.name.clone();
                self.structs.push((st_name.clone(), st));
            }
        }
    }

    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        match self.structs.iter_mut().find(|(st_name, _)| st_name == struct_name) {
            Some((_, c)) => Some(c),
            None => None
        }
    }

    fn get_struct_names(&self) -> Vec<String> {
        self.structs
            .iter()
            .map(|(st_name, _)| st_name.clone())
            .collect()
    }
}