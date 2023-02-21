use ra_ap_syntax::{ast::{self, AstNode}, match_ast};
use super::HasUMLEntity;
use crate::uml_entity::*;
use super::utils::{get_paths_str_from_ast_node, strip_trait_bound, get_fn_full_name};

impl HasUMLEntity for ast::Impl {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut results = vec![];

        // get struct name
        let mut impl_fn_names = vec![];
        let struct_name: String = strip_trait_bound(&self.self_ty().unwrap().to_string());

        let mut dep_list: Vec<String> = vec![];
        let mut asct_list: Vec<String> = vec![];
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    // get impl functions' names
                    ast::Fn(f) => {
                        impl_fn_names.push(get_fn_full_name(&f));
                    },
                    // get Dependency and Association Relations
                    ast::ParamList(pl) => {
                        dep_list.append(&mut get_paths_str_from_ast_node(pl));
                    },
                    ast::BlockExpr(ex) => {
                        dep_list.append(&mut get_paths_str_from_ast_node(ex));
                    },
                    ast::RetType(rt) => {
                        asct_list.append(&mut get_paths_str_from_ast_node(rt));
                    },
                    _ => ()
                }
            }
        }

        // first add Association Relation, then add dependency relation if the name not occured in assocaitions
        results.extend(
            asct_list.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&p, &struct_name, UMLRelationKind::UMLAssociationUni)))
        );
        let mut dep_set: Vec<&String> = dep_list.iter().filter(|p| !asct_list.contains(p)).collect();
        dep_set.sort();
        dep_set.dedup();
        results.extend(
            dep_set.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLDependency)))
        );


        // get trait if there is any
        match self.trait_() {
            Some(tt) => {
                results.push(
                    UMLEntity::UMLRelation(UMLRelation::new(&struct_name, &strip_trait_bound(&tt.to_string()), UMLRelationKind::UMLRealization))
                );
                // println!("trait: {}", tt.to_string());
            },
            None => {
                results.push(UMLEntity::UMLClass(UMLClass::new(&struct_name, vec![], impl_fn_names, UMLClassKind::UMLClass)));
            }
        }
        

        results
    }
}