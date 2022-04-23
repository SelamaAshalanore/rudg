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
}