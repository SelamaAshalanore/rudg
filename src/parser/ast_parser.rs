// The basic idea is, extend crates ra_ap_syntax, especially ast, to support UML entity methods.
use ra_ap_syntax::{ast::{self, HasModuleItem}, SourceFile, Parse};

use crate::uml_entity::*;
use super::StringParser;
use super::to_uml_entities::HasUMLEntity;

pub struct AstParser;

impl StringParser for AstParser {
    fn parse_string(input: &str) -> UMLGraph {
        // parse code string into UML Graph
        let parse: Parse<SourceFile> = SourceFile::parse(input);
        let file: SourceFile = parse.tree();
        let mut uml_graph = UMLGraph::new("");
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
                ast::Item::Use(u) => {
                    uml_entities.append(&mut u.get_uml_entities());
                }
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
                UMLEntity::UMLOuterEntity(oe) => uml_graph.add_outer_entity_new(oe),
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
        let mut target_graph: UMLGraph = UMLGraph::new("");
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
        let mut target_graph: UMLGraph = UMLGraph::new("");
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
        let mut target_graph: UMLGraph = UMLGraph::new("");

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
        let mut target_graph: UMLGraph = UMLGraph::new("");

        target_graph.add_struct(UMLClass::new("Mock", vec![], vec![String::from("mock_fn()")], UMLClassKind::UMLClass));
        target_graph.add_fn(UMLFn::new("f1", "f1(i: usize)"));
        target_graph.add_fn(UMLFn::new("f2", "f2() -> usize"));
        target_graph.add_relation(UMLRelation::new("Mock", "f1", UMLRelationKind::UMLDependency));
        target_graph.add_relation(UMLRelation::new("Mock", "f2", UMLRelationKind::UMLDependency));
        
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
        let mut target_graph: UMLGraph = UMLGraph::new("");

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
        let mut target_graph: UMLGraph = UMLGraph::new("");

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
        let mut target_graph: UMLGraph = UMLGraph::new("");

        target_graph.add_struct(UMLClass::new("A", vec![String::from(r"a: T")], vec![String::from(r"a(a: T) -> Self")], UMLClassKind::UMLClass));
        target_graph.add_struct(UMLClass::new("B", vec![], vec![String::from(r"a(&self) -> Option<T>")], UMLClassKind::UMLTrait));
        target_graph.add_relation(UMLRelation::new("A", "B", UMLRelationKind::UMLRealization));
        
        // relations with invalid end(s) are stored inside the class but cannnot be reached
        assert_eq!(parsed_graph.structs(), target_graph.structs());
        assert_eq!(parsed_graph.fns(), target_graph.fns());
        assert_eq!(parsed_graph.relations(), target_graph.relations());
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
        let mut target_graph: UMLGraph = UMLGraph::new("");

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
        let mut target_graph: UMLGraph = UMLGraph::new("");

        target_graph.add_fn(UMLFn::new("mock", "mock() -> ()"));
        target_graph.add_outer_entity("Hello", "hello");
        target_graph.add_outer_entity("hello", "hello");

        target_graph.add_relation(UMLRelation::new("mock", "hello.Hello", UMLRelationKind::UMLDependency));
        target_graph.add_relation(UMLRelation::new("mock", "hello.hello", UMLRelationKind::UMLDependency));
        
        assert_eq!(parsed_graph, target_graph);
    }

}