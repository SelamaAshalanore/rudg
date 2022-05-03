mod uml_entity;

use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName}};
use dot::{LabelledGraph, graph_to_string, edge, Edge, Style, id_name};
use uml_entity::DotEntity;


// This should not exists! Fix dot first so this will be no longer necessary
fn string_to_static_str(s: String) -> &'static str {
    return Box::leak(s.into_boxed_str())
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut func_names: Vec<&str> = vec![];
    let mut struct_names: Vec<&str> = vec![];
    let mut impl_names: Vec<(&str, &str)> = vec![];
    let mut call_dependency_names: Vec<(&str, &str)> = vec![];
    let mut dot_entities: Vec<DotEntity> = vec![];


    for item in file.items() {
        // println!("{}", &item);
        match item {
            ast::Item::Fn(f) => {
                let f_name = string_to_static_str(f.name().unwrap().text().to_string());
                func_names.push(f_name);
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
                                    call_dependency_names.push((f_name, (string_to_static_str(call_name.clone()))));
                                    dot_entities.push(DotEntity::Edge(edge(f_name, call_name.as_str(), "", Style::None, None)))
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
                            func_names.push(string_to_static_str(f.name().unwrap().text().to_string()));
                            impl_names.push((string_to_static_str(struct_name.clone()), string_to_static_str(f.name().unwrap().text().to_string())));
                            dot_entities.push(DotEntity::Label(f.name().unwrap().text().to_string()));
                            dot_entities.push(DotEntity::Edge(edge(struct_name.as_str(), f_name.as_str(), "", Style::None, None)))
                        },
                        _ => ()
                    }
                }
            },
            ast::Item::Struct(st) => {
                struct_names.push(string_to_static_str(st.name().unwrap().text().to_string()));
                dot_entities.push(DotEntity::Label(st.name().unwrap().text().to_string()));
            },
            _ => (),
        }
    }

    let mut all_names: Vec<&str> = struct_names.clone();
    all_names.append(func_names.as_mut());


    let mut edge_vec: Vec<Edge> = vec![];
    for impl_desc in impl_names {
        let struct_name = impl_desc.0;
        let func_name = impl_desc.1;
        let struct_index = all_names.iter().position(|&name| name == struct_name).unwrap();
        let func_index = all_names.iter().position(|&name| name == func_name).unwrap();
        edge_vec.push(edge(id_name(&func_index).as_slice(), id_name(&struct_index).as_slice(), "impl", Style::None, None));
    }

    for impl_desc in call_dependency_names {
        let outer_name = impl_desc.0;
        let inner_name = impl_desc.1;
        let outer_index = all_names.iter().position(|&name| name == outer_name).unwrap();
        let inner_index = all_names.iter().position(|&name| name == inner_name).unwrap();
        edge_vec.push(edge(id_name(&outer_index).as_slice(), id_name(&inner_index).as_slice(), "call", Style::None, None));
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
    

    let labels = label_list.iter().map(|name| Some(*name)).collect();
    let digraph = LabelledGraph::new("ast", labels, edge_list, None);

    return graph_to_string(digraph).unwrap();
}