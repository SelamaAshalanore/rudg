use ra_ap_syntax::{ast::{self}};
use super::HasUMLEntity;
use crate::uml_entity::*;
use super::utils::replace_coloncolon_path;

impl HasUMLEntity for ast::Use {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut outer_entities: Vec<UMLOuterEntity> = vec![];
        walk_use_tree(self.use_tree().unwrap(), &mut outer_entities, None);

        outer_entities.iter()
            .map(|oe| UMLEntity::UMLOuterEntity(oe.clone()))
            .collect()
    }
}

fn walk_use_tree(ut: ast::UseTree, outer_entities: &mut Vec<UMLOuterEntity>, path_name: Option<&str>) -> () {
    // recursivelly add sub use tree's entities
    println!("use tree: {}", ut.to_string());
    let ut_path = ut.path().unwrap().to_string();

    let current_path_name = match path_name {
        Some(pn) => vec![pn, ".", &ut_path].concat(),
        None => ut_path.clone()
    };

    match ut.use_tree_list() {
        Some(ut_list) => {
            for sub_ut in ut_list.use_trees() {
                // if current use tree has use tree list
                walk_use_tree(sub_ut, outer_entities, Some(&current_path_name));
            }
        },
        None => {
            // if not, add current use tree info and return
            match path_name {
                Some(pn) => outer_entities.push(UMLOuterEntity::new(&ut_path, pn)),
                None => {
                    let ut_path_string = replace_coloncolon_path(&ut_path);
                    let mut ut_dot_path: Vec<&str> = ut_path_string.split(".").collect();
                    let name = ut_dot_path[ut_dot_path.len()-1].clone();
                    ut_dot_path.truncate(ut_dot_path.len().saturating_sub(1));
                    let mod_name = ut_dot_path.join(".");
                    outer_entities.push(UMLOuterEntity::new(name, &mod_name))
                }
            };
        }
    }
}