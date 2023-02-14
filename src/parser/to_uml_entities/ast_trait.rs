use ra_ap_syntax::{ast::{self, AstNode, HasName}, match_ast};
use super::HasUMLEntity;
use crate::uml_entity::*;
use super::utils::get_paths_str_from_ast_node;

impl HasUMLEntity for ast::Trait {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut results = vec![];
        // add UMLClass
        results.push(UMLEntity::UMLClass(UMLClass::new(&self.name().unwrap().text().to_string(), vec![], vec![], UMLClassKind::UMLTrait)));

        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::RecordField(rf) => {
                        // get Aggregation and Composition Relations
                        let rf_str = rf.to_string();
                        if rf_str.contains(r"*mut") || rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(
                                    UMLEntity::UMLRelation(UMLRelation::new(&self.name().unwrap().text().to_string(), &p, UMLRelationKind::UMLAggregation)))
                                )
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(
                                    UMLEntity::UMLRelation(UMLRelation::new(&self.name().unwrap().text().to_string(), &p, UMLRelationKind::UMLComposition)))
                                )
                        }
                    },
                    _ => ()
                }
            }
        };

        results
    }
}