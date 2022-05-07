pub mod uml_entity;
mod uml_to_dot;
mod ast_parser;

use ra_ap_syntax::{SourceFile, Parse};
use uml_entity::{UMLModule};
use uml_to_dot::{DotEntity, UMLEntity};
use dot::{graph_to_string, new_graph};
use std::path::Path;
use std::fs::read_to_string;
use dot::{Edge, Node};

/// The function `rs2dot` returns graphed file module.
///
/// # Examples
/// ```
/// extern crate staticanalyzer;
///
/// fn main() {
///     let _ = staticanalyzer::rs2dot("src/lib.rs");
/// }
/// ```
pub fn rs2dot<'a, P: AsRef<Path>>(path: P) -> String {
    let file_string = read_to_string(path).unwrap();
    code_to_dot_digraph(&file_string)
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let mut uml_module = UMLModule::new();

    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();
    uml_module.parse_source_file(file);

    let (node_list, edge_list) = get_node_and_edge_list(uml_module.get_dot_entities());

    // generate digraph from nodes and edges
    let new_digraph = new_graph("ast", node_list, edge_list, None);

    return graph_to_string(new_digraph).unwrap();
}

pub fn get_node_and_edge_list(dot_entities: Vec<DotEntity>) -> (Vec<Node>, Vec<Edge>) {
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