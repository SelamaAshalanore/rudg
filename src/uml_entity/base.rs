use super::{UMLClass, UMLFn, UMLRelation};

pub enum UMLEntity {
    // UML Entity has three types, Class, Function and Relation
    UMLClass(UMLClass),
    UMLFn(UMLFn),
    UMLRelation(UMLRelation)
}