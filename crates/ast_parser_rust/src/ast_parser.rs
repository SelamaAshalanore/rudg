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
        vec![UMLClass::new(&self.name().unwrap().text().to_string(), record_fields, vec![])]
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
                            get_paths_str_from_record_field(rf)
                                .iter()
                                .for_each(|p| results.push(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLAggregation)))
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_record_field(rf)
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
        let impl_funcs = self.get_or_create_assoc_item_list().assoc_items();
        for impl_func in impl_funcs {
            match impl_func {
                ast::AssocItem::Fn(f) => {
                    f.get_uml_fn().iter().for_each(|f| impl_fn_names.push(f.full_name.clone()));
                },
                _ => ()
            }
        }
        vec![UMLClass::new(&struct_name, vec![], impl_fn_names)]
    }
}

impl HasUMLRelation for ast::Impl {
    fn get_uml_relations(&self) -> Vec<UMLRelation> {
        let mut dep_fn_names: Vec<String> = vec![];
        let struct_name: String = self.self_ty().unwrap().to_string();
        let impl_funcs = self.get_or_create_assoc_item_list().assoc_items();
        for impl_func in impl_funcs {
            match impl_func {
                ast::AssocItem::Fn(f) => {
                    dep_fn_names.append(&mut get_dependency_names(&f).clone());
                },
                _ => ()
            }
        }
        dep_fn_names.iter()
                    .map(|f| UMLRelation::new(&struct_name, f, UMLRelationKind::UMLDependency))
                    .collect()
    }
}

fn get_paths_str_from_record_field(rf: ast::RecordField) -> Vec<String> {
    let mut results = vec![];
    for node in rf.syntax().descendants() {
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
        let mut full_name: String = f_name.clone();

        let mut dependent_fn_names: Vec<UMLRelation> = vec![];
        // visit all Fn descendants and process CallExpr
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(UMLRelation::new(&f_name, &call_name, UMLRelationKind::UMLDependency))
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

fn get_dependency_names(f: &ast::Fn) -> Vec<String> {
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
    dependent_fn_names
}