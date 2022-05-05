mod uml_entity;

use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName, Fn}, AstNode, match_ast};
use dot::{graph_to_string, edge, Edge, Style, new_graph, Node};
use uml_entity::{DotEntity, UMLFn, UMLClass};

pub fn code_to_dot_digraph(code: &str) -> String {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut dot_entities: Vec<DotEntity> = vec![];

    // visit all items in SourceFile and extract dot entities from every type of them
    for item in file.items() {
        match item {
            ast::Item::Fn(f) => {
                let uml_fn = UMLFn::from_ast_fn(&f);
                dot_entities.append(&mut uml_fn.get_dot_entities());
            },
            ast::Item::Impl(ip) => {
                dot_entities.append(&mut get_impl_dot_entities(ip));
            },
            ast::Item::Struct(st) => {
                let uml_class = UMLClass::from_ast_struct(&st);
                dot_entities.append(&mut uml_class.get_dot_entities());
            },
            _ => (),
        }
    }

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

fn get_fn_dot_entities(f: Fn) -> Vec<DotEntity> {
    let mut dot_entities = vec![];
    let f_name = f.name().unwrap().text().to_string();
    dot_entities.push(DotEntity::Label(f_name.clone()));

    // visit all Fn descendants and process CallExpr
    for node in f.syntax().descendants() {
        match_ast! {
            match node {
                ast::CallExpr(it) => {
                    let call_name = get_call_expr_fn_names(it);
                    dot_entities.push(DotEntity::Edge(edge(&f_name, call_name.as_str(), "call", Style::None, None)))
                },
                _ => (),
            }
        }
    }
    dot_entities
}

fn get_impl_dot_entities(ip: ast::Impl) -> Vec<DotEntity> {
    let mut dot_entities = vec![];
    let struct_name: String = ip.self_ty().unwrap().to_string();
    let impl_funcs = ip.get_or_create_assoc_item_list().assoc_items();
    for impl_func in impl_funcs {
        match impl_func {
            ast::AssocItem::Fn(f) => {
                let f_name = f.name().unwrap().text().to_string();
                dot_entities.append(&mut get_fn_dot_entities(f));
                dot_entities.push(DotEntity::Edge(edge(f_name.as_str(), struct_name.as_str(), "impl", Style::None, None)))
            },
            _ => ()
        }
    }
    dot_entities
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}