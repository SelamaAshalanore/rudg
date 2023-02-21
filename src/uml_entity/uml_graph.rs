use {
    super::uml_fn::UMLFn,
    super::{UMLClass},
    super::{UMLRelation, UMLRelationKind},
};
use std::collections::BTreeMap;

use super::{UMLClassKind, UMLOuterEntity};

#[derive(PartialEq, Debug)]
pub struct UMLGraph {
    // The Graph struct, one for each graph, contains all the information on it
    pub name: String,
    structs: Vec<UMLClass>,
    fns: Vec<UMLFn>,
    relations: Vec<UMLRelation>,
    pub modules: BTreeMap<String, UMLGraph>,
    outer_entities: Vec<UMLOuterEntity>,
}

impl UMLGraph {
    // Getters
    pub fn fns(&self) -> Vec<&UMLFn> {
        // functions getter
        self.fns
            .iter()
            .filter(|f| !f.name.contains("."))
            .collect()
    }

    pub fn structs(&self) -> Vec<&UMLClass> {
        // structs getter
        self.structs
            .iter()
            .filter(|cls| !cls.name.contains("."))
            .collect()
    }

    pub fn relations(&self) -> Vec<&UMLRelation> {
        // relations getter
        self.relations
            .iter()
            .filter(|rel| {
                (self.get_fn_names().contains(&rel.from) || self.get_struct_names().contains(&rel.from)) &&
                (self.get_fn_names().contains(&rel.to) || self.get_struct_names().contains(&rel.to)) &&
                (!rel.to.contains(r".") && !rel.from.contains(r"."))
            })
            .collect()
    }
    pub fn outer_relations_new(&self) -> Vec<UMLRelation> {
        // outer relations getter
        self.relations
            .iter()
            .filter(|rel| {
                self.is_outer_entity(&rel.from) || self.is_outer_entity(&rel.to)
            })
            .map(|rel| {
                let mut rel_results = rel.clone();
                let from_name = self.get_outer_entity_full_name(&rel.from);
                let to_name = self.get_outer_entity_full_name(&rel.to);
                rel_results.update_relation_names(&from_name, &to_name);
                rel_results
            })
            .collect()
    }

    pub fn outer_relations(&self) -> Vec<&UMLRelation> {
        // outer relations getter
        self.relations
            .iter()
            .filter(|rel| {
                (self.get_fn_names().contains(&rel.from) || self.get_struct_names().contains(&rel.from)) &&
                (self.get_fn_names().contains(&rel.to) || self.get_struct_names().contains(&rel.to)) &&
                (rel.to.contains(r".") || rel.from.contains(r"."))
            })
            .collect()
    }

    pub fn outer_entities(&self) -> Vec<&UMLOuterEntity> {
        // outer entites getter
        self.outer_entities
            .iter()
            .collect()
    }

    fn get_struct_names(&self) -> Vec<String> {
        // struct names getter
        self.structs
            .iter()
            .map(|st| st.name.clone())
            .collect()
    }

    fn get_fn_names(&self) -> Vec<String> {
        // function names getter
        self.fns
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }
}

impl UMLGraph {
    // Finders
    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        // mut structs getter
        self.structs.iter_mut().find(|st| st.name == struct_name)
    }

    fn relation_mut(&mut self, from: &str, to: &str) -> Option<&mut UMLRelation> {
        // relation mut getter
        for rel in &mut self.relations {
            if rel.from == from && rel.to == to {
                return Some(rel)
            }
        }
        None
    }

    fn is_outer_entity(&self, name: &str) -> bool {
        self.outer_entities
            .iter()
            .map(|oe| oe.name.clone())
            .collect::<String>()
            .contains(name)
    }

    fn get_outer_entity_full_name(&self, name: &str) -> String {
        match self.is_outer_entity(name) {
            false => String::from(name),
            true => {
                match self.outer_entities 
                    .iter()
                    .find(|oe| oe.name == name) {
                        Some(soe) => vec![soe.mod_name.as_str(), ".", name].concat(),
                        None => String::from(name)
                    }
            }
        }
    }
}

impl UMLGraph {
    // Setters & Adders
    pub fn new(name: &str) -> UMLGraph {
        UMLGraph { name: String::from(name), structs: vec![], fns: vec![], relations: vec![], modules: BTreeMap::new(), outer_entities: vec![]}
    }

    pub fn add_module(&mut self, module: UMLGraph) -> () {
        self.modules.insert(String::from(&module.name), module);
    }

    pub fn add_relation(&mut self, rel: UMLRelation) -> () {
        // if relation's from or to not in graph already, it cannot be added
        if &rel.from != &rel.to {
            // if new relation's kind is associationUni, then search for associationUni relation with opposite direction and replace it with associationBi
            if &rel.kind == &UMLRelationKind::UMLAssociationUni {
                match self.relation_mut(&rel.to, &rel.from) {
                    Some(e_rel) => {
                        if &e_rel.kind == &rel.kind {
                            e_rel.change_relation_kind(UMLRelationKind::UMLAssociationBi);
                            return
                        }
                    },
                    None => ()
                }
            }
            
            match self.relation_mut(&rel.from, &rel.to) {
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
        // add struct, if exists, extend its methods
        if self.get_struct_names().contains(&cls.name) {
            self.get_mut_struct(&cls.name).unwrap().merge_method_names_from(&mut cls.clone());
        } else {
            self.structs.push(cls);
        }
    }

    pub fn add_outer_class(&mut self, cls_name: &str, kind: UMLClassKind, mod_name: &str) -> () {
        self.structs.push(UMLClass::new(&format!("{}.{}", mod_name, cls_name), vec![], vec![], kind));
    }

    pub fn add_fn(&mut self, f: UMLFn) -> () {
        self.fns.push(f);
    }

    pub fn add_outer_fn(&mut self, f_name: &str, mod_name: &str) -> () {
        self.fns.push(UMLFn::new(&format!("{}.{}", mod_name, f_name), &format!("{}.{}", mod_name, f_name)));
    }

    pub fn add_outer_entity(&mut self, outer_entity: UMLOuterEntity) -> () {
        self.outer_entities.push(outer_entity);
    }
}