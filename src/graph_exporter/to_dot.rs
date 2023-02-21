mod uml_fn;
mod uml_class;
mod uml_relation;
mod uml_graph;

use dot_graph::{Edge, Node, Graph, Subgraph};
use crate::uml_entity::*;

use super::GraphExporter;
enum DotEntity {
    Edge(Edge),
    Node(Node)
}

trait HasDotEntity {
    fn get_dot_entities(&self, name_prefix: &str) -> Vec<DotEntity>;
}

impl GraphExporter for UMLGraph {
    fn to_string(&self) -> String {
        let (node_list, edge_list) = get_node_and_edge_list(self.get_dot_entities(""));

        // generate digraph from nodes and edges
        let mut graph = Graph::new("ast", dot_graph::Kind::Digraph);
        for node in node_list {
            graph.add_node(node)
        }
        for edge in edge_list {
            graph.add_edge(edge)
        }

        // generate digraph from modules
        for (name, m) in &self.modules {
            let name_prefix = format!("{}.", &m.name);
            let (node_list, edge_list) = get_node_and_edge_list(m.get_dot_entities(&name_prefix));
            let mut subgraph = Subgraph::new(&format!("cluster_{}", name)).label(name);
            subgraph.add_nodes(node_list);
            edge_list.iter().for_each(|e| subgraph.add_edge(e.clone()));
            graph.add_subgraph(subgraph);

            // add outer_relations
            for r in m.outer_relations() {
                for ent in r.get_dot_entities(&name_prefix) {
                    match ent {
                        DotEntity::Edge(e) => graph.add_edge(e),
                        DotEntity::Node(n) => graph.add_node(n)
                    }
                }
            }
        }

        return graph.to_dot_string().unwrap();
    }
}

fn get_node_and_edge_list(dot_entities: Vec<DotEntity>) -> (Vec<Node>, Vec<Edge>) {
    // transform DotEntity to nodes and edges that 'dot' can use
    // let mut label_list: Vec<&str> = vec![];
    let mut edge_list: Vec<Edge> = vec![];
    let mut node_list: Vec<Node> = vec![];
    for ent in dot_entities {
        match ent {
            DotEntity::Edge(ent_edge) => {
                edge_list.push(ent_edge);
            },
            DotEntity::Node(node) => {
                node_list.push(node);
            },
        }
    }
    (node_list, edge_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uml_classes_fns_to_dot() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Main", vec![String::from("a: String"), String::from("b: String")], vec![String::from("main() -> ()"), String::from("main1()")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("MainTrait", vec![], vec![String::from("main() -> ()")], UMLClassKind::UMLTrait));
        uml_graph.add_fn(UMLFn::new("test", "test()"));

        let dot_string = uml_graph.to_string();
        let target_string = r#"digraph ast {
    "Main"[label="{Main|a: String\lb: String|main() -> ()\lmain1()}"][shape="record"];
    "MainTrait"[label="{Interface\lMainTrait|main() -> ()}"][shape="record"];
    "test"[label="test"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_fn_relations() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_fn(UMLFn::new("main", "main()"));
        uml_graph.add_fn(UMLFn::new("test", "test()"));
        uml_graph.add_relation(UMLRelation::new("main", "test", UMLRelationKind::UMLDependency));

        let dot_string = uml_graph.to_string();
        let target_string = r#"digraph ast {
    "main"[label="main"];
    "test"[label="test"];
    "main" -> "test"[label=""][style="dashed"][arrowhead="vee"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_dependency() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("mock_fn()")], UMLClassKind::UMLClass));
        uml_graph.add_fn(UMLFn::new("f1", "f1(i: usize)"));
        uml_graph.add_fn(UMLFn::new("f2", "f2() -> usize"));
        uml_graph.add_relation(UMLRelation::new("f1", "Mock", UMLRelationKind::UMLDependency));
        uml_graph.add_relation(UMLRelation::new("f2", "Mock", UMLRelationKind::UMLDependency));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    "Mock"[label="{Mock|mock_fn()}"][shape="record"];
    "f1"[label="f1"];
    "f2"[label="f2"];
    "f1" -> "Mock"[label=""][style="dashed"][arrowhead="vee"];
    "f2" -> "Mock"[label=""][style="dashed"][arrowhead="vee"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_aggregation() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![String::from(r"b: *mut B")], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("B", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("Mock", "B", UMLRelationKind::UMLAggregation));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    "Mock"[label="{Mock|b: *mut B}"][shape="record"];
    "B"[label="B"][shape="record"];
    "Mock" -> "B"[label=""][arrowtail="odiamond"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_composition() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![String::from(r"c: C")], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("C", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("Mock", "C", UMLRelationKind::UMLComposition));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    "Mock"[label="{Mock|c: C}"][shape="record"];
    "C"[label="C"][shape="record"];
    "Mock" -> "C"[label=""][arrowhead="diamond"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_realization() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("D", vec![], vec![String::from(r"a(&self) -> Option<T>")], UMLClassKind::UMLTrait));
        uml_graph.add_relation(UMLRelation::new("Mock", "D", UMLRelationKind::UMLRealization));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    "Mock"[label="Mock"][shape="record"];
    "D"[label="{Interface\lD|a(&self) -> Option<T>}"][shape="record"];
    "Mock" -> "D"[label=""][style="dashed"][arrowhead="onormal"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_association() {
        let mut uml_graph = UMLGraph::new("");
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("e2() -> E2")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("E1", vec![], vec![String::from(r"b() -> Mock")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("E2", vec![], vec![String::from(r"a() -> Mock")], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("E1", "Mock", UMLRelationKind::UMLAssociationUni));
        uml_graph.add_relation(UMLRelation::new("E2", "Mock", UMLRelationKind::UMLAssociationBi));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    "Mock"[label="{Mock|e2() -> E2}"][shape="record"];
    "E1"[label="{E1|b() -> Mock}"][shape="record"];
    "E2"[label="{E2|a() -> Mock}"][shape="record"];
    "E1" -> "Mock"[label=""][arrowhead="vee"];
    "E2" -> "Mock"[label=""][arrowhead="none"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_mods() {
        let mut uml_graph = UMLGraph::new("");
        let mut uml_mod = UMLGraph::new("mock_mod");
        uml_mod.add_struct(UMLClass::new("Mock", vec![], vec![String::from("e2() -> E2")], UMLClassKind::UMLClass));
        uml_mod.add_struct(UMLClass::new("E1", vec![], vec![String::from(r"b() -> Mock")], UMLClassKind::UMLClass));
        uml_mod.add_struct(UMLClass::new("E2", vec![], vec![String::from(r"a() -> Mock")], UMLClassKind::UMLClass));
        uml_mod.add_relation(UMLRelation::new("E1", "Mock", UMLRelationKind::UMLAssociationUni));
        uml_mod.add_relation(UMLRelation::new("E2", "Mock", UMLRelationKind::UMLAssociationBi));
        uml_graph.add_module(uml_mod);

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    subgraph cluster_mock_mod {
        label="mock_mod";
        "mock_mod.Mock"[label="{Mock|e2() -> E2}"][shape="record"];
        "mock_mod.E1"[label="{E1|b() -> Mock}"][shape="record"];
        "mock_mod.E2"[label="{E2|a() -> Mock}"][shape="record"];
        "mock_mod.E1" -> "mock_mod.Mock"[label=""][arrowhead="vee"];
        "mock_mod.E2" -> "mock_mod.Mock"[label=""][arrowhead="none"];
    }
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_cross_mods_dependency() {
        let mut uml_graph = UMLGraph::new("");
        let mut uml_mod1 = UMLGraph::new("hello_mod");
        let mut uml_mod2 = UMLGraph::new("mock_mod");
        uml_mod1.add_struct(UMLClass::new("Hello", vec![], vec![], UMLClassKind::UMLClass));
        uml_mod1.add_fn(UMLFn::new("hello", "hello() -> ()"));
        uml_mod2.add_fn(UMLFn::new("mock", "mock() -> ()"));
        uml_mod2.add_outer_entity(UMLOuterEntity::new("Hello", "hello_mod"));
        uml_mod2.add_outer_entity(UMLOuterEntity::new("hello", "hello_mod"));
        uml_mod2.add_relation(UMLRelation::new("mock", "Hello", UMLRelationKind::UMLDependency));
        uml_mod2.add_relation(UMLRelation::new("mock", "hello", UMLRelationKind::UMLDependency));
        uml_graph.add_module(uml_mod1);
        uml_graph.add_module(uml_mod2);

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    subgraph cluster_hello_mod {
        label="hello_mod";
        "hello_mod.Hello"[label="Hello"][shape="record"];
        "hello_mod.hello"[label="hello"];
    }
    subgraph cluster_mock_mod {
        label="mock_mod";
        "mock_mod.mock"[label="mock"];
        "mock_mod.mock" -> "hello_mod.Hello"[label=""][style="dashed"][arrowhead="vee"];
        "mock_mod.mock" -> "hello_mod.hello"[label=""][style="dashed"][arrowhead="vee"];
    }
}
"#;
        assert_eq!(dot_string, target_string);
    }
}