
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
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("main", "Mock", UMLRelationKind::UMLDependency));
        uml_graph.add_relation(UMLRelation::new("outer_main", "Mock", UMLRelationKind::UMLDependency));
        // add relation when at least one side of the relation is not in the scope
        assert_eq!(uml_graph.relations().len(), 0);

        // relations can be added if both sides on the scope
        uml_graph.add_fn(UMLFn::new("main", "main()"));
        assert_eq!(uml_graph.relations().len(), 1);
        assert_eq!(uml_graph.relations().get(0).unwrap().from, "main");
    }

    #[test]
    fn test_add_outer_relations() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Main", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("Main", "mock.outer_mock", UMLRelationKind::UMLDependency));
        // add relation when at least one side of the relation is not in the scope
        assert_eq!(uml_graph.relations().len(), 0);
        assert_eq!(uml_graph.outer_relations().len(), 0);

        // outer relations use notation as "mod"."struct/fn name"
        uml_graph.add_outer_fn("outer_mock", "mock");
        assert_eq!(uml_graph.outer_relations().len(), 1);
        assert_eq!(uml_graph.relations().len(), 0);
        assert_eq!(uml_graph.outer_relations().get(0).unwrap().from, "Main");
        assert_eq!(uml_graph.outer_relations().get(0).unwrap().to, "mock.outer_mock");
    }
}
