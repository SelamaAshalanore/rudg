use super::{HasDotEntity, DotEntity};
use crate::uml_entity::*;

impl HasDotEntity for UMLGraph {
    fn get_dot_entities(&self, name_prefix: &str) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        self.structs()
            .iter()
            .for_each(|st| dot_entities.append(&mut st.get_dot_entities(name_prefix)));
        self.fns()
            .iter()
            .for_each(|f| dot_entities.append(&mut f.get_dot_entities(name_prefix)));
        self.relations()
            .iter()
            .for_each(|r| dot_entities.append(&mut r.get_dot_entities(name_prefix)));
        self.outer_relations()
            .iter()
            .for_each(|r| {
                dot_entities.append(&mut r.get_dot_entities(name_prefix))
            });
        dot_entities
    }
}