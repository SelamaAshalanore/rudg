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
            dep_set.iter().map(|p| UMLEntity::UMLRelation(UMLRelation::new(&p, &struct_name, UMLRelationKind::UMLDependency)))
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
        let mut uml_graph = UMLGraph::new();
        let mut uml_entities: Vec<UMLEntity> = vec![];

        // visit all items in SourceFile and extract dot entities from every type of them
        for item in file.items() {
            match item {
                ast::Item::Fn(f) => {
                    uml_entities.append(&mut f.get_uml_entities());
                },
                ast::Item::Impl(ip) => {
                    uml_entities.append(&mut ip.get_uml_entities());
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

        // add relations last
        let mut relations: Vec<UMLRelation> = vec![];
        for e in uml_entities {
            match e {
                UMLEntity::UMLClass(c) => uml_graph.add_struct(c),
                UMLEntity::UMLFn(f) => uml_graph.add_fn(f),
                UMLEntity::UMLRelation(r) => {
                    // uml_graph.add_relation(r);
                    relations.push(r);
                },
            }
        }
        for rel in relations {
            uml_graph.add_relation(rel);
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

    #[test]
    fn test_parse_struct() {
        let code: &str = r#"
        pub struct Mock;
            impl Mock {
                pub fn mock_fn() {}
            }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();
        target_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("mock_fn()")], UMLClassKind::UMLClass));
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_fn_dependency() {
        let code: &str = r#"
            fn main() {
                hello();
            }
            fn hello() {}
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_fn(UMLFn::new("main", "main()"));
        target_graph.add_fn(UMLFn::new("hello", "hello()"));
        target_graph.add_relation(UMLRelation::new("main", "hello", UMLRelationKind::UMLDependency));
        
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_class_dependency() {
        let code: &str = r#"
        pub struct Mock;
        impl Mock {
            pub fn mock_fn() { f1(f2()) }    
        }
        fn f1(i: usize) {}
        fn f2() -> usize { 0 }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("mock_fn()")], UMLClassKind::UMLClass));
        target_graph.add_fn(UMLFn::new("f1", "f1(i: usize)"));
        target_graph.add_fn(UMLFn::new("f2", "f2() -> usize"));
        target_graph.add_relation(UMLRelation::new("f1", "Mock", UMLRelationKind::UMLDependency));
        target_graph.add_relation(UMLRelation::new("f2", "Mock", UMLRelationKind::UMLDependency));
        
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_aggregation() {
        let code: &str = r#"
        struct Amut {
            b: *mut B,
        }
        
        struct Aconst {
            b: *const B,
        }
        
        struct B {
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_struct(UMLClass::new("Amut", vec![String::from(r"b: *mut B")], vec![], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("Aconst", vec![String::from(r"b: *const B")], vec![], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("B", vec![], vec![], UMLClassKind::UMLClass));
        target_graph.add_relation(UMLRelation::new("Amut", "B", UMLRelationKind::UMLAggregation));
        target_graph.add_relation(UMLRelation::new("Aconst", "B", UMLRelationKind::UMLAggregation));
        
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_composition() {
        let code: &str = r#"
        struct A {
            b: B,
        }
        
        struct B {
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_struct(UMLClass::new("A", vec![String::from(r"b: B")], vec![], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("B", vec![], vec![], UMLClassKind::UMLClass));
        target_graph.add_relation(UMLRelation::new("A", "B", UMLRelationKind::UMLComposition));
        
        assert_eq!(parsed_graph, target_graph);
    }

    
    #[test]
    fn test_realization() {
        let code: &str = r#"
        use std::fmt::Debug;

        #[derive(Debug)]
        struct A<T> where T: Debug {
            a: T,
        }

        impl<T> A<T> where T: Debug {
            fn a(a: T) -> Self {
                A {
                    a: a,
                }
            }
        }

        impl <T>B<T> for A<T> where T: Debug {
            fn a(&self) -> Option<T> {
                None
            }
        }

        trait B<T> : Debug where T: Debug {
            fn a(&self) -> Option<T>;
        }

        impl <T>B<T> {
            fn a(&self) -> Option<T> {
                None
            }
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_struct(UMLClass::new("A", vec![String::from(r"a: T")], vec![String::from(r"a(a: T) -> Self")], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("B", vec![], vec![String::from(r"a(&self) -> Option<T>")], UMLClassKind::UMLTrait));
        target_graph.add_relation(UMLRelation::new("A", "B", UMLRelationKind::UMLRealization));
        
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_association() {
        let code: &str = r#"
        struct A {
        }
        
        impl A {
            fn b() -> B {
                B {
                }
            }
        }
        
        struct Ab {
        }
        
        impl Ab {
            fn b() -> B {
                B {
                }
            }
        }
        
        struct B {
        }
        
        impl B {
            fn a() -> Ab {
                Ab {
                }
            }
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_struct(UMLClass::new("A", vec![], vec![String::from(r"b() -> B")], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("Ab", vec![], vec![String::from(r"b() -> B")], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("B", vec![], vec![String::from(r"a() -> Ab")], UMLClassKind::UMLClass));
        target_graph.add_relation(UMLRelation::new("B", "A", UMLRelationKind::UMLAssociationUni));
        target_graph.add_relation(UMLRelation::new("B", "Ab", UMLRelationKind::UMLAssociationBi));
        
        assert_eq!(parsed_graph, target_graph);
    }

    #[test]
    fn test_outer_structs() {
        let code: &str = r#"
        use hello::{Hello, hello};

        fn mock() -> () {
            Hello::new();
            hello();
        }
        "#;
        let parsed_graph = AstParser::parse_string(code);
        let mut target_graph: UMLGraph = UMLGraph::new();

        target_graph.add_fn(UMLFn::new("mock", "mock() -> ()"));
        target_graph.add_outer_class("Hello", UMLClassKind::UMLClass, "hello");
        target_graph.add_outer_fn("hello", "hello");
        
        assert_eq!(parsed_graph, target_graph);
    }

}