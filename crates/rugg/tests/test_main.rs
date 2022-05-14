#[cfg(test)]
mod tests {
    use rugg::{code_to_dot_digraph};

    #[test]
    fn parse_simple_r_code() {
        let code: &str = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
}
"#
        )
    }

    
    #[test]
    fn parse_class_with_fn() {
        let code: &str = r#"
            pub struct Mock;
            impl Mock {
                pub fn mock_fn() {}
            }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    Mock[label="{Mock|mock_fn()}"][shape="record"];
}
"#
        )
    }

    #[test]
    fn parse_class_with_nested_impl() {
        let code: &str = r#"
            pub struct Mock;
            impl Mock {
                pub fn mock_fn() { f1(f2()) }    
            }
            fn f1(i: usize) {}
            fn f2() -> usize { 0 }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    Mock[label="{Mock|mock_fn()}"][shape="record"];
    f1[label="f1"];
    f2[label="f2"];
    f1 -> Mock[label=""][style="dashed"][arrowhead="vee"];
    f2 -> Mock[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_fn_dependency() {
        let code: &str = r#"
            fn main() {
                hello();
            }
            fn hello() {}
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    hello[label="hello"];
    main -> hello[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_fns_dependency() {
        let code: &str = r#"
            fn main() {
                f1();
                f2();
            }
            fn f1() {}
            fn f2() {}
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    f1[label="f1"];
    f2[label="f2"];
    main -> f1[label=""][style="dashed"][arrowhead="vee"];
    main -> f2[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_nested_fn_call_dependency() {
        let code: &str = r#"
            fn main() {
                f1(f2());
            }
            fn f1(i: usize) {}
            fn f2() -> usize { 0 }
        "#;
        assert_eq!(
            code_to_dot_digraph(code), 
r#"digraph ast {
    main[label="main"];
    f1[label="f1"];
    f2[label="f2"];
    main -> f1[label=""][style="dashed"][arrowhead="vee"];
    main -> f2[label=""][style="dashed"][arrowhead="vee"];
}
"#
        )
    }

    #[test]
    fn test_aggregation() {
        assert_eq!(
            rugg::rs2dot("tests/examples/aggregation.rs"),
r#"digraph ast {
    Amut[label="{Amut|b: *mut B}"][shape="record"];
    Aconst[label="{Aconst|b: *const B}"][shape="record"];
    B[label="B"][shape="record"];
    Amut -> B[label=""][arrowtail="odiamond"];
    Aconst -> B[label=""][arrowtail="odiamond"];
}
"#
        )
    }

    #[test]
    fn test_association() {
        assert_eq!(
            rugg::rs2dot("tests/examples/association.rs"),
r#"digraph ast {
    A[label="{A|b() -> B}"][shape="record"];
    Ab[label="{Ab|b() -> B}"][shape="record"];
    B[label="{B|a() -> Ab}"][shape="record"];
    B -> A[label=""][arrowhead="vee"];
    B -> Ab[label=""][arrowhead="none"];
}
"#
        )
    }

    #[test]
    fn test_composition() {
        assert_eq!(
            rugg::rs2dot("tests/examples/composition.rs"),
r#"digraph ast {
    A[label="{A|b: B}"][shape="record"];
    B[label="B"][shape="record"];
    A -> B[label=""][arrowhead="diamond"];
}
"#
    );
    }

    #[test]
    fn test_dependency() {
        assert_eq!(
            rugg::rs2dot("tests/examples/dependency.rs"),
r#"digraph ast {
    A[label="{A|b(b: &B)}"][shape="record"];
    B[label="B"][shape="record"];
    B -> A[label=""][style="dashed"][arrowhead="vee"];
}
"#
    );
    }

    #[test]
    fn test_realization() {
        assert_eq!(
            rugg::rs2dot("tests/examples/realization.rs"),
r#"digraph ast {
    A[label="{A|a: T|a(a: T) -> Self}"][shape="record"];
    B[label="{Interface\lB|a(&self) -> Option<T>}"][shape="record"];
    A -> B[label=""][style="dashed"][arrowhead="onormal"];
}
"#
    );
    }

}