mod uml_entity;

use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName}};
use dot::{graph_to_string, edge, Edge, Style, new_graph, Node};
use uml_entity::DotEntity;


// This should not exists! Fix dot first so this will be no longer necessary
fn string_to_static_str(s: String) -> &'static str {
    return Box::leak(s.into_boxed_str())
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut dot_entities: Vec<DotEntity> = vec![];


    for item in file.items() {
        // println!("{}", &item);
        match item {
            ast::Item::Fn(f) => {
                let f_name = string_to_static_str(f.name().unwrap().text().to_string());
                dot_entities.push(DotEntity::Label(f_name.to_string()));
                for stmt in f.get_or_create_body().statements() {
                    match stmt {
                        ast::Stmt::ExprStmt(expr) => {
                            let exp = expr.expr().unwrap();
                            match exp {
                                ast::Expr::CallExpr(call_exp) => {
                                    let call_expr = call_exp.to_string();
                                    let call_names: Vec<&str> = call_expr.split("(").collect();
                                    let call_name = String::from(call_names[0]);
                                    dot_entities.push(DotEntity::Edge(edge(f_name, call_name.as_str(), "call", Style::None, None)))
                                },
                                _ => ()
                            }
                            
                        },
                        _ => ()
                    };
                    
                }
            },
            ast::Item::Impl(ip) => {
                let struct_name: String = ip.self_ty().unwrap().to_string();
                let impl_funcs = ip.get_or_create_assoc_item_list().assoc_items();
                for impl_func in impl_funcs {
                    match impl_func {
                        ast::AssocItem::Fn(f) => {
                            let f_name = f.name().unwrap().text().to_string();
                            dot_entities.push(DotEntity::Label(f.name().unwrap().text().to_string()));
                            dot_entities.push(DotEntity::Edge(edge(f_name.as_str(), struct_name.as_str(), "impl", Style::None, None)))
                        },
                        _ => ()
                    }
                }
            },
            ast::Item::Struct(st) => {
                dot_entities.push(DotEntity::Label(st.name().unwrap().text().to_string()));
            },
            _ => (),
        }
    }

    let mut label_list: Vec<&str> = vec![];
    let mut edge_list: Vec<Edge> = vec![];
    for ent in dot_entities {
        match ent {
            DotEntity::Label(ent_s) => {
                label_list.push(string_to_static_str(ent_s));
            },
            DotEntity::Edge(ent_edge) => {
                edge_list.push(ent_edge);
            }
        }
    }
    let node_list: Vec<Node> = label_list
                                .iter()
                                .map(|l| Node::new(l, l, Style::None, None))
                                .collect();    
    let new_digraph = new_graph("ast", node_list, edge_list, None);

    return graph_to_string(new_digraph).unwrap();
}