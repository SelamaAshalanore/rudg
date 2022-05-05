mod uml_entity;

use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem}};
use dot::{graph_to_string, Edge, Style, new_graph, Node};
use uml_entity::{DotEntity, UMLFn, UMLClass, UMLModule};

pub fn code_to_dot_digraph(code: &str) -> String {
    let mut uml_module = UMLModule::new();

    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    // visit all items in SourceFile and extract dot entities from every type of them
    for item in file.items() {
        match item {
            ast::Item::Fn(f) => {
                let uml_fn = UMLFn::from_ast_fn(&f);
                uml_module.add_fn(uml_fn);
            },
            ast::Item::Impl(ip) => {
                uml_module.add_ast_impl(ip);
            },
            ast::Item::Struct(st) => {
                let uml_class = UMLClass::from_ast_struct(&st);
                uml_module.add_struct(uml_class);
            },
            _ => (),
        }
    }

    let dot_entities = uml_module.get_dot_entities();

    // transform DotEntity to nodes and edges that 'dot' can use
    // let mut label_list: Vec<&str> = vec![];
    let mut edge_list: Vec<Edge> = vec![];
    let mut node_list: Vec<Node> = vec![];
    for ent in dot_entities {
        match ent {
            DotEntity::Label(ent_s) => {
                let node = Node::new(&ent_s, &ent_s, Style::None, None);
                node_list.push(node);
            },
            DotEntity::Edge(ent_edge) => {
                edge_list.push(ent_edge);
            }
        }
    }

    // generate digraph from nodes and edges
    let new_digraph = new_graph("ast", node_list, edge_list, None);

    return graph_to_string(new_digraph).unwrap();
}