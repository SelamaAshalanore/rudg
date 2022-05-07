use ra_ap_syntax::{ast::{self, AstNode, HasName}, match_ast};
use crate::uml_entity::*;

pub trait HasUMLFn {
    fn get_uml_fn(&self) -> Vec<UMLFn>;
}

impl HasUMLFn for ast::Fn {
    fn get_uml_fn(&self) -> Vec<UMLFn> {
        let f_name = self.name().unwrap().text().to_string();
        let mut full_name: String = f_name.clone();

        let mut dependent_fn_names = vec![];
        // visit all Fn descendants and process CallExpr
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(call_name)
                    },
                    ast::ParamList(pl) => {
                        full_name.push_str(&pl.to_string());
                    },
                    _ => {
                        // println!("{:?}", node);
                        // println!("{}", node)
                    },
                }
            }
        }
        
        vec![UMLFn { name: f_name, dependent_fn_names: dependent_fn_names, full_name: full_name}]
    }
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}