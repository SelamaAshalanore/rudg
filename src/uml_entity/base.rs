use super::{UMLClass, UMLFn, UMLRelation, UMLOuterEntity};

pub enum UMLEntity {
    // UML Entity has three types, Class, Function and Relation
    UMLClass(UMLClass),
    UMLFn(UMLFn),
    UMLRelation(UMLRelation),
    UMLOuterEntity(UMLOuterEntity),
}