use dot::{Edge, edge, Style};
use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName, Fn}, AstNode, match_ast};

pub enum DotEntity {
    Edge(Edge),
    Label(String)
}

pub struct UMLFn {
    name: String,
    dependent_fn_names: Vec<String>
}

impl UMLFn {
    pub fn from_ast_fn(f: &ast::Fn) -> UMLFn {
        let f_name = f.name().unwrap().text().to_string();


        let mut dependent_fn_names = vec![];
        // visit all Fn descendants and process CallExpr
        for node in f.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(call_name)
                    },
                    _ => (),
                }
            }
        }
        
        UMLFn { name: f_name, dependent_fn_names: dependent_fn_names }
    }

    pub fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Label(self.name.clone()));

        self.dependent_fn_names
            .iter()
            .for_each(|f_name| dot_entities.push(DotEntity::Edge(edge(&self.name, f_name, "call", Style::None, None))));

        dot_entities
    }
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}