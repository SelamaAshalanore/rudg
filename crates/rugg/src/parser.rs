pub mod ast_parser;

use crate::UMLGraph;

pub trait StringParser {
    fn parse_string(input: &str) -> UMLGraph;
}