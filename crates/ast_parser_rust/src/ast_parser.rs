use ra_ap_syntax::{ast::{self, AstNode, HasName}, match_ast};
use crate::uml_entity::*;

pub trait HasUMLFn {
    fn get_uml_fn(&self) -> Vec<UMLFn>;
}

pub trait HasUMLClass {
    fn get_uml_class(&self) -> Vec<UMLClass>;
}

pub trait HasUMLRelation {
    fn get_uml_relations(&self) -> Vec<UMLRelation>;
}

impl HasUMLClass for ast::Struct {
    fn get_uml_class(&self) -> Vec<UMLClass> {
        let mut record_fields = vec![];
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::RecordField(rf) => {
                        record_fields.push(rf.to_string())
                    },
                    _ => ()
                }
            }
            // println!("{:?}", node);
            // println!("{}", node);
        };
        vec![UMLClass::new(&self.name().unwrap().text().to_string(), record_fields, vec![], UMLClassKind::UMLClass)]
    }
}

impl HasUMLRelation for ast::Struct {
    fn get_uml_relations(&self) -> Vec<UMLRelation> {
        let mut results = vec![];
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::RecordField(rf) => {
                        let rf_str = rf.to_string();
                        if rf_str.contains(r"*mut") || rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLAggregation)))
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLComposition)))
                        }
                    },
                    _ => ()
                }
            }
        };
        results
    }
}

impl HasUMLClass for ast::Trait {
    fn get_uml_class(&self) -> Vec<UMLClass> {
        vec![UMLClass::new(&self.name().unwrap().text().to_string(), vec![], vec![], UMLClassKind::UMLTrait)]
    }
}

impl HasUMLRelation for ast::Trait {
    fn get_uml_relations(&self) -> Vec<UMLRelation> {
        let mut results = vec![];
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::RecordField(rf) => {
                        let rf_str = rf.to_string();
                        if rf_str.contains(r"*mut") || rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLAggregation)))
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLComposition)))
                        }
                    },
                    _ => ()
                }
            }
        };
        results
    }
}

impl HasUMLClass for ast::Impl {
    fn get_uml_class(&self) -> Vec<UMLClass> {
        let mut impl_fn_names = vec![];
        let struct_name: String = self.self_ty().unwrap().to_string();
        let class_name: Vec<&str> = struct_name.split(r"<").collect();
        
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::Fn(f) => {
                        f.get_uml_fn().iter().for_each(|f| impl_fn_names.push(f.full_name.clone()));
                    },
                    _ => ()
                }
            }
        }
        // println!("get UMLClass from impl with name: {} and fn_names: {:?}", class_name[0], impl_fn_names);
        vec![UMLClass::new(class_name[0], vec![], impl_fn_names, UMLClassKind::Unknown)]
    }
}

impl HasUMLRelation for ast::Impl {
    fn get_uml_relations(&self) -> Vec<UMLRelation> {
        let mut results: Vec<UMLRelation> = vec![];
        let struct_name: String = self.self_ty().unwrap().to_string();
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::ParamList(pl) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(pl);
                        results.extend(
                            path_names.iter().map(|p| UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLDependency))
                        );
                    },
                    ast::BlockExpr(ex) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(ex);
                        results.extend(
                            path_names.iter().map(|p| UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLDependency))
                        );
                    },
                    ast::RetType(rt) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(rt);
                        results.extend(
                            path_names.iter().map(|p| UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLAssociationUni))
                        );
                    },
                    _ => (),
                }
            }
            // println!("{:?}", node);
            // println!("{}", node);
        }
        results
    }
}

fn get_paths_str_from_ast_node(node: impl ast::AstNode) -> Vec<String> {
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
    };
    results
}

impl HasUMLRelation for ast::Fn {
    fn get_uml_relations(&self) -> Vec<UMLRelation> {
        let f_name = self.name().unwrap().text().to_string();

        let mut dependent_fn_names: Vec<UMLRelation> = vec![];
        // visit all Fn descendants and process CallExpr
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(UMLRelation::new(&f_name, &call_name, UMLRelationKind::UMLDependency))
                    },
                    _ => {
                        // println!("{:?}", node);
                        // println!("{}", node)
                    },
                }
            }
        }
        
        dependent_fn_names
    }
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
        
        vec![UMLFn::new(&f_name, &full_name)]
    }
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}