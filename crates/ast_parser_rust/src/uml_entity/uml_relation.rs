
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UMLRelationKind {
    UMLDependency=0,
    UMLAssociationUni=1,
    UMLAssociationBi=2,
    UMLAggregation=3,
    UMLComposition=4,
    UMLRealization=5
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

    pub fn same_objects(&self, other: &UMLRelation) -> bool {
        self.from == other.from && self.to == other.to
    }

    pub fn opposite_objects(&self, other: &UMLRelation) -> bool {
        self.from == other.to && self.to == other.from
    }
}