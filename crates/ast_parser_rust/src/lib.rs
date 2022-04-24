use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName}};
use dot::{LabelledGraph, NodeLabels, graph_to_string};


// This should not exists! Fix dot first so this will be no longer necessary
fn string_to_static_str(s: String) -> &'static str {
    return Box::leak(s.into_boxed_str())
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut func_names: Vec<&str> = vec![];
    let mut struct_names: Vec<&str> = vec![];
    let mut impl_names: Vec<&str> = vec![];


    for item in file.items() {
        println!("{}", &item);
        match item {
            ast::Item::Fn(f) => {
                func_names.push(string_to_static_str(f.name().unwrap().text().to_string()));
            },
            ast::Item::Impl(ip) => {
                let impl_funcs = ip.assoc_item_list().unwrap().assoc_items();
                for impl_func in impl_funcs {
                    match impl_func {
                        ast::AssocItem::Fn(f) => {
                            func_names.push(string_to_static_str(f.name().unwrap().text().to_string()));
                        },
                        _ => ()
                    }
                }
            },
            ast::Item::Struct(st) => {
                struct_names.push(string_to_static_str(st.name().unwrap().text().to_string()));
            },
            _ => (),
        }
    }

    let mut all_names: Vec<&str> = struct_names.clone();
    all_names.append(func_names.as_mut());
    all_names.append(impl_names.as_mut());

    let labels = NodeLabels::AllNodesLabelled(all_names);
    let digraph = LabelledGraph::new("ast", labels, vec![], None);

    return graph_to_string(digraph).unwrap();
}