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
    Mock[label="Mock"];
    mock_fn[label="mock_fn"];
    mock_fn -> Mock[label="impl"];
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
    main -> hello[label="call"];
}
"#
        )
    }
}