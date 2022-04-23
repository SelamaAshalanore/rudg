use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName}};
use dot::{LabelledGraph, NodeLabels, graph_to_string};


// This should not exists! Fix dot first so this will be no longer necessary
fn string_to_static_str(s: String) -> &'static str {
    return Box::leak(s.into_boxed_str())
}

pub fn code_to_dot_digraph(code: &str) -> String {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut func = None;
    for item in file.items() {
        match item {
            ast::Item::Fn(f) => func = Some(f),
            _ => unreachable!(),
        }
    }
    let func_name: String = func.unwrap().name().unwrap().text().to_string();
    let labels = NodeLabels::AllNodesLabelled(vec!(string_to_static_str(func_name)));
    let digraph = LabelledGraph::new("ast", labels, vec![], None);

    return graph_to_string(digraph).unwrap();
}