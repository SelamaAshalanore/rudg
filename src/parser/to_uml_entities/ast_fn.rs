use ra_ap_syntax::{ast::{self, AstNode, HasName}, match_ast};
use super::HasUMLEntity;
use crate::uml_entity::*;
use super::utils::{get_fn_full_name, get_call_expr_fn_names, replace_coloncolon_path};

impl HasUMLEntity for ast::Fn {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut results: Vec<UMLEntity> = vec![];
        let f_name = self.name().unwrap().text().to_string();
        let full_name: String = get_fn_full_name(self);

        // visit all Fn descendants and process CallExpr
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        results.push(UMLEntity::UMLRelation(UMLRelation::new(&f_name, &replace_coloncolon_path(&call_name), UMLRelationKind::UMLDependency)))
                    },
                    _ => {
                        // println!("{:?}", node);
                        // println!("{}", node)
                    },
                }
            }
        }
        results.push(UMLEntity::UMLFn(UMLFn::new(&f_name, &full_name)));
        results
    }
}