use ra_ap_syntax::{ast::{self, HasModuleItem}, SourceFile};

use crate::ast_parser::{HasUMLFn, HasUMLClass, HasUMLRelation};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UMLRelationKind {
    UMLDependency=0,
    UMLAssociationUni=1,
    UMLAssociationBi=2,
    UMLAggregation=3,
    UMLComposition=4,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UMLRelation {
    pub from: String,
    pub to: String,
    pub kind: UMLRelationKind
}

impl UMLRelation {
    pub fn new(from: &str, to: &str, kind: UMLRelationKind) -> UMLRelation {
        UMLRelation { from: String::from(from), to: String::from(to), kind: kind }
    }

    fn same_objects(&self, other: &UMLRelation) -> bool {
        self.from == other.from && self.to == other.to
    }
}

pub struct UMLFn {
    pub name: String,
    pub full_name: String
}

impl UMLFn {
    pub fn new(name: &str, full_name: &str) -> UMLFn {
        UMLFn { name: String::from(name), full_name: String::from(full_name) }
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
    relations: Vec<UMLRelation>
}

impl UMLModule {
    pub fn new() -> UMLModule {
        UMLModule { structs: vec![], fns: vec![], relations: vec![]}
    }

    pub fn parse_source_file(&mut self, src_file: SourceFile) -> () {
        // visit all items in SourceFile and extract dot entities from every type of them
        for item in src_file.items() {
            match item {
                ast::Item::Fn(f) => {
                    self.fns.append(&mut f.get_uml_fn());
                    self.add_relations(&mut f.get_uml_relations());
                },
                ast::Item::Impl(ip) => {
                    self.add_structs(ip.get_uml_class());
                    self.add_relations(&mut ip.get_uml_relations());
                },
                ast::Item::Struct(st) => {
                    self.add_structs(st.get_uml_class());
                    self.add_relations(&mut st.get_uml_relations());
                },
                _ => (),
            }
        }
    }

    fn add_relations(&mut self, rel_list: &mut Vec<UMLRelation>) -> () {
        self.relations.append(rel_list);
    }

    pub fn get_relations(&self) -> Vec<UMLRelation> {
        let mut relations = self.relations.clone();
        relations.sort();
        relations.reverse();

        let mut results: Vec<UMLRelation> = vec![];
        for r in relations {
            match results.last() {
                Some(r_other) => if r.same_objects(r_other) {
                    results.push(r);
                },
                None => { results.push(r) }
            }
        }
        results
    }

    fn add_structs(&mut self, st_list: Vec<UMLClass>) -> () {
        for mut st in st_list {
            if self.get_struct_names().contains(&st.name) {
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