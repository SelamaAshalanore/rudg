extern crate staticanalyzer;

#[test]
fn test_dependency() {
    assert_eq!(
        staticanalyzer::rs2dot("tests/examples/dependency.rs"),
        r#"digraph ast {
    A[label="{A|b(b: &B)}"][shape="record"];
    B[label="B"][shape="record"];
    B -> A[label=""][style="dashed"][arrowhead="vee"];
}
"#);
}