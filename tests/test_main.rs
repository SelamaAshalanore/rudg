#[cfg(test)]
mod tests {
    use staticanalyzer::parse_code;

    #[test]
    fn parse_simple_r_code() {
        let code: &str = r#"
        fn main() {
            println!("Hello, world!");
        }        
        "#;
        let result: Vec<String> = vec![String::from("main")];
        assert_eq!(parse_code(code), result)
    }
}