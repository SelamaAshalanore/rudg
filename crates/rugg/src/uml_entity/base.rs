use super::{UMLClass, UMLFn, UMLRelation};

pub enum UMLEntity {
    UMLClass(UMLClass),
    UMLFn(UMLFn),
    UMLRelation(UMLRelation)
}