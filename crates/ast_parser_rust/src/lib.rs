mod uml_entity;

use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem}};
use dot::{graph_to_string, new_graph};
use uml_entity::{UMLFn, UMLClass, UMLModule};

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

    let (node_list, edge_list) = uml_module.get_node_and_edge_list();

    // generate digraph from nodes and edges
    let new_digraph = new_graph("ast", node_list, edge_list, None);

    return graph_to_string(new_digraph).unwrap();
}