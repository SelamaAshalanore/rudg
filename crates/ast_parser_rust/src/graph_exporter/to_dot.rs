
use dot::{Edge, Style, Node, edge_with_arrows, Arrow, ArrowShape, Fill, Side};
use crate::uml_entity::*;
use dot::{graph_to_string, new_graph};

use super::GraphExporter;
pub enum DotEntity {
    Edge(Edge),
    Node(Node)
}

pub trait UMLEntity {
    fn get_dot_entities(&self) -> Vec<DotEntity>;
}

impl UMLEntity for UMLFn {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &self.name, Style::None, None, None)));
        dot_entities
    }
}


impl UMLEntity for UMLClass {
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


impl UMLEntity for UMLGraph {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        self.structs
            .iter()
            .for_each(|(_, st)| dot_entities.append(&mut st.get_dot_entities()));
        self.fns
            .iter()
            .for_each(|f| dot_entities.append(&mut f.get_dot_entities()));
        self.get_relations()
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

impl UMLEntity for UMLRelation {
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