#[cfg(test)]
mod tests {
    use staticanalyzer::code_to_dot_digraph;

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
    N0[label="main"];
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
    N0[label="Mock"];
    N1[label="mock_fn"];
    N1 -> N0[label="impl"];
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
    N0[label="main"];
    N1[label="hello"];
    N0 -> N1[label="call"];
}
"#
        )
    }
}