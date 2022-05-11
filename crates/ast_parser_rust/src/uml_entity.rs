
mod uml_fn;
mod uml_class;
mod uml_relation;
mod uml_module;

pub use {
    uml_fn::UMLFn,
    uml_class::{UMLClass, UMLClassKind},
    uml_relation::{UMLRelation, UMLRelationKind},
    uml_module::UMLModule
};

