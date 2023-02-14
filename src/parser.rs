pub mod ast_parser;
mod to_uml_entities;

use crate::UMLGraph;

pub trait StringParser {
    fn parse_string(input: &str) -> UMLGraph;
}