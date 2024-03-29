
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum UMLRelationKind {
    // UML relation types
    UMLDependency=0,
    UMLAssociationUni=1,
    UMLAssociationBi=2,
    UMLAggregation=3,
    UMLComposition=4,
    UMLRealization=5
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct UMLRelation {
    pub from: String,
    pub to: String,
    pub kind: UMLRelationKind
}

impl UMLRelation {
    pub fn new(from: &str, to: &str, kind: UMLRelationKind) -> UMLRelation {
        UMLRelation { from: String::from(from), to: String::from(to), kind: kind }
    }

    pub fn change_relation_kind(&mut self, new_relation_kind: UMLRelationKind) -> () {
        self.kind = new_relation_kind
    }

    pub fn update_relation_names(&mut self, from: &str, to: &str) -> () {
        self.from = String::from(from);
        self.to = String::from(to);
    }
}