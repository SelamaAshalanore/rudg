
use dot_graph::{Edge, Style, Node, edge_with_arrows, Arrow, ArrowShape, Fill, Side};
use crate::uml_entity::*;
use dot_graph::{graph_to_string, new_graph};

use super::GraphExporter;
enum DotEntity {
    Edge(Edge),
    Node(Node)
}

trait HasDotEntity {
    fn get_dot_entities(&self) -> Vec<DotEntity>;
}

impl HasDotEntity for UMLFn {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &self.name, Style::None, None, None)));
        dot_entities
    }
}


impl HasDotEntity for UMLClass {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        match self.kind {
            UMLClassKind::UMLClass => {
                let mut label_text: Vec<&str> = vec![&self.name];
                let method_names = self.get_method_names();
                let field_names = self.get_field_names();

                let method_names_str = method_names.join(r"\l");
                let field_names_str = field_names.join(r"\l");
                if method_names.len() + field_names.len() > 0 {
                    label_text.insert(0, "{");
                    if field_names.len() > 0 {
                        label_text.push("|");
                        label_text.push(&field_names_str);
                    } 
                    if method_names.len() > 0 {
                        label_text.push("|");
                        label_text.push(&method_names_str);    
                    }
                    label_text.push("}");
                }
                let label: String = label_text.into_iter().collect();
                
                dot_entities.push(DotEntity::Node(Node::new(&self.name, &label, Style::None, None, Some(String::from("record")))));
                
            },
            UMLClassKind::UMLTrait => {
                let mut label_text: Vec<&str> = vec![r"Interface\l", &self.name];
                let method_names = self.get_method_names();

                let method_names_str = method_names.join(r"\l");
                if method_names.len() > 0 {
                    label_text.insert(0, "{");
                    label_text.push("|");
                    label_text.push(&method_names_str);
                    label_text.push("}");
                }
                let label: String = label_text.into_iter().collect();
                
                dot_entities.push(DotEntity::Node(Node::new(&self.name, &label, Style::None, None, Some(String::from("record")))));
            },
        }

        dot_entities
    }
}


impl HasDotEntity for UMLGraph {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        self.structs
            .iter()
            .for_each(|(_, st)| dot_entities.append(&mut st.get_dot_entities()));
        self.fns
            .iter()
            .for_each(|f| dot_entities.append(&mut f.get_dot_entities()));
        self.relations
            .iter()
            .for_each(|r| dot_entities.append(&mut r.get_dot_entities()));
        dot_entities
    }
}

impl GraphExporter for UMLGraph {
    fn to_string(&self) -> String {
        let (node_list, edge_list) = get_node_and_edge_list(self.get_dot_entities());

        // generate digraph from nodes and edges
        let new_digraph = new_graph("ast", node_list, edge_list, None);

        return graph_to_string(new_digraph).unwrap();
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

impl HasDotEntity for UMLRelation {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        match self.kind {
            UMLRelationKind::UMLAggregation => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from, 
                    &self.to, 
                    "", 
                    Style::None, 
                    Arrow::from_arrow(ArrowShape::Diamond(Fill::Open, Side::Both)),
                    Arrow::default(),
                    None
                ))]
            },
            UMLRelationKind::UMLComposition => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from, 
                    &self.to, 
                    "",
                    Style::None,
                    Arrow::default(),
                    Arrow::from_arrow(ArrowShape::diamond()),
                    None
                ))]
            },
            UMLRelationKind::UMLDependency => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from,
                    &self.to, 
                    "",
                    Style::Dashed,
                    Arrow::default(),
                    Arrow::from_arrow(ArrowShape::vee()),
                    None
                ))]
            },
            UMLRelationKind::UMLAssociationUni => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from,
                    &self.to,
                    "",
                    Style::None,
                    Arrow::default(),
                    Arrow::from_arrow(ArrowShape::vee()),
                    None
                ))]
            },
            UMLRelationKind::UMLAssociationBi => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from, 
                    &self.to, 
                    "",
                    Style::None,
                    Arrow::default(),
                    Arrow::none(),
                    None
                ))]
            },
            UMLRelationKind::UMLRealization => {
                vec![DotEntity::Edge(edge_with_arrows(
                    &self.from, 
                    &self.to, 
                    "",
                    Style::Dashed,
                    Arrow::default(),
                    Arrow::from_arrow(ArrowShape::Normal(Fill::Open, Side::Both)),
                    None
                ))]
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uml_classes_fns_to_dot() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Main", vec![String::from("a: String"), String::from("b: String")], vec![String::from("main() -> ()"), String::from("main1()")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("MainTrait", vec![], vec![String::from("main() -> ()")], UMLClassKind::UMLTrait));
        uml_graph.add_fn(UMLFn::new("test", "test()"));

        let dot_string = uml_graph.to_string();
        let target_string = r#"digraph ast {
    Main[label="{Main|a: String\lb: String|main() -> ()\lmain1()}"][shape="record"];
    MainTrait[label="{Interface\lMainTrait|main() -> ()}"][shape="record"];
    test[label="test"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_fn_relations() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_fn(UMLFn::new("main", "main()"));
        uml_graph.add_fn(UMLFn::new("test", "test()"));
        uml_graph.add_relation(UMLRelation::new("main", "test", UMLRelationKind::UMLDependency));

        let dot_string = uml_graph.to_string();
        let target_string = r#"digraph ast {
    main[label="main"];
    test[label="test"];
    main -> test[label=""][style="dashed"][arrowhead="vee"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_dependency() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("mock_fn()")], UMLClassKind::UMLClass));
        uml_graph.add_fn(UMLFn::new("f1", "f1(i: usize)"));
        uml_graph.add_fn(UMLFn::new("f2", "f2() -> usize"));
        uml_graph.add_relation(UMLRelation::new("f1", "Mock", UMLRelationKind::UMLDependency));
        uml_graph.add_relation(UMLRelation::new("f2", "Mock", UMLRelationKind::UMLDependency));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    Mock[label="{Mock|mock_fn()}"][shape="record"];
    f1[label="f1"];
    f2[label="f2"];
    f1 -> Mock[label=""][style="dashed"][arrowhead="vee"];
    f2 -> Mock[label=""][style="dashed"][arrowhead="vee"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_aggregation() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![String::from(r"b: *mut B")], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("B", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("Mock", "B", UMLRelationKind::UMLAggregation));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    Mock[label="{Mock|b: *mut B}"][shape="record"];
    B[label="B"][shape="record"];
    Mock -> B[label=""][arrowtail="odiamond"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_composition() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![String::from(r"c: C")], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("C", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("Mock", "C", UMLRelationKind::UMLComposition));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    Mock[label="{Mock|c: C}"][shape="record"];
    C[label="C"][shape="record"];
    Mock -> C[label=""][arrowhead="diamond"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_realization() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("D", vec![], vec![String::from(r"a(&self) -> Option<T>")], UMLClassKind::UMLTrait));
        uml_graph.add_relation(UMLRelation::new("Mock", "D", UMLRelationKind::UMLRealization));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    Mock[label="Mock"][shape="record"];
    D[label="{Interface\lD|a(&self) -> Option<T>}"][shape="record"];
    Mock -> D[label=""][style="dashed"][arrowhead="onormal"];
}
"#;
        assert_eq!(dot_string, target_string);
    }

    #[test]
    fn test_uml_class_association() {
        let mut uml_graph = UMLGraph::new();
        uml_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("e2() -> E2")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("E1", vec![], vec![String::from(r"b() -> Mock")], UMLClassKind::UMLClass));
        uml_graph.add_struct(UMLClass::new("E2", vec![], vec![String::from(r"a() -> Mock")], UMLClassKind::UMLClass));
        uml_graph.add_relation(UMLRelation::new("E1", "Mock", UMLRelationKind::UMLAssociationUni));
        uml_graph.add_relation(UMLRelation::new("E2", "Mock", UMLRelationKind::UMLAssociationBi));

        let dot_string = uml_graph.to_string();
        let target_string = 
r#"digraph ast {
    Mock[label="{Mock|e2() -> E2}"][shape="record"];
    E1[label="{E1|b() -> Mock}"][shape="record"];
    E2[label="{E2|a() -> Mock}"][shape="record"];
    E1 -> Mock[label=""][arrowhead="vee"];
    E2 -> Mock[label=""][arrowhead="none"];
}
"#;
        assert_eq!(dot_string, target_string);
    }
}