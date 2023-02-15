use super::{HasDotEntity, DotEntity};
use crate::uml_entity::*;
use dot_graph::{Node};

impl HasDotEntity for UMLFn {
    fn get_dot_entities(&self, name_prefix: &str) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        let name = vec![name_prefix, &self.name];
        dot_entities.push(DotEntity::Node(Node::new(&name.concat()).label(&self.name)));
        dot_entities
    }
}