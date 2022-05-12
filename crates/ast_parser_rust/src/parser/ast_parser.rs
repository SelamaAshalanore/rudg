use ra_ap_syntax::{ast::{self, AstNode, HasName, HasModuleItem}, match_ast, SourceFile, Parse};

use crate::uml_entity::*;
use super::StringParser;

trait HasUMLEntity {
    fn get_uml_entities(&self) -> Vec<UMLEntity>;
}

impl HasUMLEntity for ast::Struct {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut results = vec![];
        let mut record_fields = vec![];
        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    ast::RecordField(rf) => {
                        // get fields for UMLClass
                        record_fields.push(rf.to_string());

                        // get Aggregation and Composition Relations
                        let rf_str = rf.to_string();
                        if rf_str.contains(r"*mut") || rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(
                                    UMLEntity::UMLRelation(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLAggregation)))
                                )
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(
                                    UMLEntity::UMLRelation(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLComposition)))
                                )
                        }
                    },
                    _ => ()
                }
            }
            // println!("{:?}", node);
            // println!("{}", node);
        };
        results.push(UMLEntity::UMLClass(UMLClass::new(&self.name().unwrap().text().to_string(), record_fields, vec![], UMLClassKind::UMLClass)));
        results
    }
}

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
                                    UMLEntity::UMLRelation(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLAggregation)))
                                )
                        } else if !rf_str.contains(r"*mut") && !rf_str.contains(r"*const") {
                            get_paths_str_from_ast_node(rf)
                                .iter()
                                .for_each(|p| results.push(
                                    UMLEntity::UMLRelation(UMLRelation::new(&p, &self.name().unwrap().text().to_string(), UMLRelationKind::UMLComposition)))
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

impl HasUMLEntity for ast::Impl {
    fn get_uml_entities(&self) -> Vec<UMLEntity> {
        let mut results = vec![];

        // get struct name
        let mut impl_fn_names = vec![];
        let struct_name: String = strip_trait_bound(&self.self_ty().unwrap().to_string());

        // get trait if there is any
        match self.trait_() {
            Some(tt) => {
                results.push(
                    UMLEntity::UMLRelation(UMLRelation::new(&strip_trait_bound(&tt.to_string()), &struct_name, UMLRelationKind::UMLRealization))
                );
                println!("trait: {}", tt.to_string());
            },
            None => ()
        }

        for node in self.syntax().descendants() {
            match_ast! {
                match node {
                    // get impl functions' names
                    ast::Fn(f) => {
                        impl_fn_names.push(get_fn_full_name(&f));
                        // f.get_uml_fn().iter().for_each(|f| impl_fn_names.push(get_fn_full_name(f)));
                    },
                    // get Dependency and Association Relations
                    ast::ParamList(pl) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(pl);
                        results.extend(
                            path_names.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLDependency)))
                        );
                    },
                    ast::BlockExpr(ex) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(ex);
                        results.extend(
                            path_names.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLDependency)))
                        );
                    },
                    ast::RetType(rt) => {
                        let path_names: Vec<String> = get_paths_str_from_ast_node(rt);
                        results.extend(
                            path_names.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&struct_name, &p, UMLRelationKind::UMLAssociationUni)))
                        );
                    },
                    _ => ()
                }
            }
        }
        results.push(UMLEntity::UMLClass(UMLClass::new(&struct_name, vec![], impl_fn_names, UMLClassKind::Unknown)));

        results
    }
}

fn strip_trait_bound(s: &str) -> String {
    let class_name: Vec<&str> = s.split(r"<").collect();
    String::from(class_name[0])
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
        // println!("{:?}", node);
        // println!("{}", node);
    };
    results
}

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
                        results.push(UMLEntity::UMLRelation(UMLRelation::new(&f_name, &call_name, UMLRelationKind::UMLDependency)))
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

fn get_fn_full_name(f: &ast::Fn) -> String {
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

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}

pub struct AstParser;

impl StringParser for AstParser {
    fn parse_string(input: &str) -> UMLGraph {
        let parse: Parse<SourceFile> = SourceFile::parse(input);
        let file: SourceFile = parse.tree();
        // parsing impls after all other nodes have been parsed
        let mut uml_graph = UMLGraph::new();
        let mut uml_entities: Vec<UMLEntity> = vec![];
        let mut impl_entities: Vec<UMLEntity> = vec![];


        // visit all items in SourceFile and extract dot entities from every type of them
        for item in file.items() {
            match item {
                ast::Item::Fn(f) => {
                    uml_entities.append(&mut f.get_uml_entities());
                },
                ast::Item::Impl(ip) => {
                    impl_entities.append(&mut ip.get_uml_entities());
                },
                ast::Item::Struct(st) => {
                    uml_entities.append(&mut st.get_uml_entities());
                },
                ast::Item::Trait(tt) => {
                    uml_entities.append(&mut tt.get_uml_entities());
                },
                _ => (),
            }
        }

        for e in uml_entities {
            match e {
                UMLEntity::UMLClass(c) => uml_graph.add_struct(c),
                UMLEntity::UMLFn(f) => uml_graph.add_fn(f),
                UMLEntity::UMLRelation(r) => uml_graph.add_relation(r),
            }
        }
        
        for e in impl_entities {
            match e {
                UMLEntity::UMLClass(c) => uml_graph.add_impl_classes(vec![c]),
                UMLEntity::UMLFn(f) => uml_graph.add_fn(f),
                UMLEntity::UMLRelation(r) => uml_graph.add_relation(r),
            }
        }

        uml_graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fn() {
        let code: &str = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();
        target_graph.add_fn(UMLFn::new("main", "main()"));
        assert_eq!(parsed_graph, target_graph);
    }

    // #[test]
    // fn test_parse_struct() {
    //     let code: &str = r#"
    //     pub struct Mock;
    //         impl Mock {
    //             pub fn mock_fn() {}
    //         }
    //     "#;
    //     let parsed_graph = AstParser::parse_string(code);
    //     let mut target_graph: UMLGraph = UMLGraph::new();
    //     target_graph.add_fn(UMLFn::new("main", "main()"));
    //     assert_eq!(parsed_graph, target_graph);
    // }

}