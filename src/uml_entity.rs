
mod uml_fn;
mod uml_class;
mod uml_relation;
mod uml_graph;
mod base;

pub use {
    uml_fn::UMLFn,
    uml_class::{UMLClass, UMLClassKind},
    uml_relation::{UMLRelation, UMLRelationKind},
    uml_graph::UMLGraph,
    base::UMLEntity
};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_relations() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("main", "Mock", UMLRelationKind::UMLDependency));
        // add relation when at least one side of the relation is not in the scope
        assert_eq!(uml_graph.relations().len(), 0);

        // relations can be added if both sides on the scope
        uml_graph.add_fn(UMLFn::new("main", "main()"));
        uml_graph.add_relation(UMLRelation::new("main", "Mock", UMLRelationKind::UMLDependency));
        assert_eq!(uml_graph.relations().len(), 1);
    }
}
