use std::ops::Index;

use ra_ap_syntax::{ast::{self, HasModuleItem}, SourceFile, AstNode, match_ast};

use crate::ast_parser::{HasUMLFn, HasUMLClass, HasUMLRelation};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UMLRelationKind {
    UMLDependency=0,
    UMLAssociationUni=1,
    UMLAssociationBi=2,
    UMLAggregation=3,
    UMLComposition=4,
}

pub enum UMLClassKind {
    UMLClass,
    UMLTrait,
    Unknown,
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

    fn opposite_objects(&self, other: &UMLRelation) -> bool {
        self.from == other.to && self.to == other.from
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
        // parsing impls after all other nodes have been parsed
        let mut impls: Vec<ast::Impl> = vec![];


        // visit all items in SourceFile and extract dot entities from every type of them
        for item in src_file.items() {
            match item {
                ast::Item::Fn(f) => {
                    self.fns.append(&mut f.get_uml_fn());
                    self.add_relations(&mut f.get_uml_relations());
                },
                ast::Item::Impl(ip) => {
                    impls.push(ip);
                },
                ast::Item::Struct(st) => {
                    self.add_structs(st.get_uml_class());
                    self.add_relations(&mut st.get_uml_relations());
                },
                ast::Item::Trait(tt) => {
                    self.add_structs(tt.get_uml_class());
                    self.add_relations(&mut tt.get_uml_relations());
                },
                _ => (),
            }
        }

        impls.iter()
            .for_each(|ip| {
                self.add_impl_classes(ip.get_uml_class());
                self.add_relations(&mut ip.get_uml_relations());
            })

    }

    fn add_relations(&mut self, rel_list: &mut Vec<UMLRelation>) -> () {
        self.relations.append(rel_list);
    }

    pub fn get_relations(&self) -> Vec<UMLRelation> {
        let mut relations = self.relations.clone();

        // compare two adjacent relation, if they have same "from" and "to", then the less ordered Relation will not count in
        relations.sort();
        relations.reverse();
        let mut results: Vec<UMLRelation> = vec![];
        for r in relations {
            match results.last() {
                Some(r_other) => if !r.same_objects(r_other) {
                    results.push(r);
                },
                None => { results.push(r) }
            }
        }
        
        self.merge_association(results)
    }

    fn merge_association(&self, relations: Vec<UMLRelation>) -> Vec<UMLRelation> {
        let mut results = vec![];
        // temp vec for storing association relations
        let mut uni_associations: Vec<UMLRelation> = vec![];
        for r in relations {
            match r.kind {
                // compare relation with Uni Association Type with every Relation in uni_associations,
                // if match with opposite relation, push Bi-Association to Results and remove matched relation from uni_associations,
                // if not, push the relation to uni_associations
                UMLRelationKind::UMLAssociationUni => {
                    let mut match_bi_index: Option<usize> = None;
                    for ua_index in 0..uni_associations.len() {
                        if r.opposite_objects(uni_associations.index(ua_index)) {
                            match_bi_index = Some(ua_index);
                            break;
                        }
                    }
                    match match_bi_index {
                        Some(i) => {
                            results.push(UMLRelation::new(&r.from, &r.to, UMLRelationKind::UMLAssociationBi));
                            uni_associations.remove(i);
                        },
                        None => {
                            uni_associations.push(r);
                        }
                    }
                },
                _ => { results.push(r) }
            }
        }

        // finally merge uni_associations to include unmatched association relations
        results.append(&mut uni_associations);
        results
    }

    fn add_structs(&mut self, st_list: Vec<UMLClass>) -> () {
        for st in st_list {
            if self.get_struct_names().contains(&st.name) {
                println!("struct or trait with name {} exists!", st.name);
            } else {
                let st_name = st.name.clone();
                self.structs.push((st_name.clone(), st));
            }
        }
    }

    fn add_impl_classes(&mut self, ip_list: Vec<UMLClass>) -> () {
        for mut ip in ip_list {
            if self.get_struct_names().contains(&ip.name) {
                self.get_mut_struct(&ip.name).unwrap().merge_from(&mut ip);
            } else {
                println!("no struct or trait with name: {}", ip.name);
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