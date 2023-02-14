use ra_ap_syntax::{ast::{self, AstNode, HasName}, match_ast};

pub fn get_paths_str_from_ast_node(node: impl ast::AstNode) -> Vec<String> {
    // get raw relation string
    let mut results = vec![];
    for node in node.syntax().descendants() {
        match_ast! {
            match node {
                ast::Path(p) => {
                    results.push(p.to_string())
                },
                _ => ()
            }
        }
        // println!("{:?}", node);
        // println!("{}", node);
    };
    results
}

pub fn strip_trait_bound(s: &str) -> String {
    let class_name: Vec<&str> = s.split(r"<").collect();
    String::from(class_name[0])
}

pub fn get_fn_full_name(f: &ast::Fn) -> String {
    // include param list, return type
    let f_name = f.name().unwrap().text().to_string();
    let mut full_name: String = f_name.clone();

    // visit all Fn descendants and process CallExpr
    for node in f.syntax().descendants() {
        match_ast! {
            match node {
                ast::ParamList(pl) => {
                    full_name.push_str(&pl.to_string());
                },
                ast::RetType(rt) => {
                    full_name.push_str(" ");
                    full_name.push_str(&rt.to_string());
                },
                _ => {
                    // println!("{:?}", node);
                    // println!("{}", node)
                },
            }
        }
    }
    full_name
}

pub fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}
