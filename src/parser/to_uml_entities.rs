use crate::uml_entity::UMLEntity;

mod utils;
mod ast_struct;
mod ast_trait;
mod ast_impl;
mod ast_fn;
mod ast_use;

pub trait HasUMLEntity {
    fn get_uml_entities(&self) -> Vec<UMLEntity>; // get uml entities from all types of ast entities
}
