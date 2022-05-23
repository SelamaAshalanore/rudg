use {
    super::uml_fn::UMLFn,
    super::{UMLClass},
    super::{UMLRelation, UMLRelationKind},
};
use std::collections::BTreeMap;

use super::UMLClassKind;

#[derive(PartialEq, Debug)]
pub struct UMLGraph {
    pub structs: Vec<UMLClass>,
    pub fns: Vec<UMLFn>,
    relations: Vec<UMLRelation>,
    pub modules: BTreeMap<String, UMLGraph>,
    outer_structs: Vec<(UMLClass, String)>,
    outer_fns: Vec<(UMLFn, String)>,
}

impl UMLGraph {
    pub fn new() -> UMLGraph {
        UMLGraph { structs: vec![], fns: vec![], relations: vec![], modules: BTreeMap::new(), outer_fns: vec![], outer_structs: vec![]}
    }

    pub fn add_module(&mut self, module: UMLGraph, name: &str) -> () {
        self.modules.insert(String::from(name), module);
    }

    pub fn add_relation(&mut self, rel: UMLRelation) -> () {
        // if relation's from or to not in graph already, it cannot be added
        if &rel.from != &rel.to {
                    // if new relation's kind is associationUni, then search for associationUni relation with opposite direction and replace it with associationBi
                    if &rel.kind == &UMLRelationKind::UMLAssociationUni {
                        match self.get_relation(&rel.to, &rel.from) {
                            Some(e_rel) => {
                                if &e_rel.kind == &rel.kind {
                                    e_rel.change_relation_kind(UMLRelationKind::UMLAssociationBi);
                                    return
                                }
                            },
                            None => ()
                        }
                    }
                    
                    match self.get_relation(&rel.from, &rel.to) {
                        Some(e_rel) => {
                            // if existing relation's kind has less priority than new relation's, change the relation kind
                            if e_rel.kind < rel.kind {
                                e_rel.change_relation_kind(rel.kind);
                            }
                        },
                        None => {
                            self.relations.push(rel);
                        }
                    }                    
                }
        else {
            dbg!("warning: this graph cannot add Relation now", &rel);
        }
        
    }

    pub fn add_struct(&mut self, cls: UMLClass) -> () {
        if self.get_struct_names().contains(&cls.name) {
            self.get_mut_struct(&cls.name).unwrap().merge_from(&mut cls.clone());
        } else {
            self.structs.push(cls);
        }
    }

    pub fn add_outer_class(&mut self, cls_name: &str, kind: UMLClassKind, mod_name: &str) -> () {
        self.outer_structs.push((UMLClass::new(cls_name, vec![], vec![], kind), String::from(mod_name)));
    }

    pub fn add_fn(&mut self, f: UMLFn) -> () {
        self.fns.push(f);
    }

    pub fn add_outer_fn(&mut self, f_name: &str, mod_name: &str) -> () {
        self.outer_fns.push((UMLFn::new(f_name, ""), String::from(mod_name)));
    }

    pub fn relations(&self) -> Vec<&UMLRelation> {
        self.relations
            .iter()
            .filter(|rel| {
                (self.get_fn_names().contains(&rel.from) || self.get_struct_names().contains(&rel.from)) &&
                (self.get_fn_names().contains(&rel.to) || self.get_struct_names().contains(&rel.to))
            })
            .collect()
    }

    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        self.structs.iter_mut().find(|st| st.name == struct_name)
    }

    fn get_struct_names(&self) -> Vec<String> {
        let mut results = vec![];
        results.append(
            &mut self.structs
                    .iter()
                    .map(|st| st.name.clone())
                    .collect()
        );
        results.append(
            &mut self.outer_structs
                    .iter()
                    .map(|(st, _)| st.name.clone())
                    .collect()
        );
        results
        
    }

    fn get_fn_names(&self) -> Vec<String> {
        let mut results = vec![];
        results.append(
            &mut self.fns
                    .iter()
                    .map(|f| f.name.clone())
                    .collect()
        );
        results.append(
            &mut self.outer_fns
                    .iter()
                    .map(|(f, _)| f.name.clone())
                    .collect()
        );
        results
    }

    fn get_relation(&mut self, from: &str, to: &str) -> Option<&mut UMLRelation> {
        for rel in &mut self.relations {
            if rel.from == from && rel.to == to {
                return Some(rel)
            }
        }
        None
    }
}