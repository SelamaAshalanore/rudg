pub mod uml_entity;
pub mod parser;
pub mod graph_exporter;

use uml_entity::{UMLGraph};
use graph_exporter::{GraphExporter};
use std::path::Path;
use std::fs::read_to_string;
use parser::{ast_parser::AstParser, StringParser};

/// The function `rs2dot` returns graphed file module.
///
/// # Examples
/// ```
/// extern crate rudg;
///
/// fn main() {
///     let _ = rudg::rs2dot("src/lib.rs");
/// }
/// ```
pub fn rs2dot<'a, P: AsRef<Path>>(path: P) -> String {
    let file_string = read_to_string(path).unwrap();
    code_to_dot_digraph(&file_string)
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let uml_graph = AstParser::parse_string(code);
    uml_graph.to_string()
}