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
        let result: String = String::from("main");
        assert_eq!(code_to_dot_digraph(code), result)
    }
}