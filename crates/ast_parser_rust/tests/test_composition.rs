extern crate staticanalyzer;

#[test]
fn test_composition() {
    assert_eq!(
        staticanalyzer::rs2dot("tests/examples/composition.rs"),
r#"digraph ast {
    A[label="{A|b: B}"][shape="record"];
    B[label="B"][shape="record"];
    B -> A[label=""][arrowhead="diamond"];
}
"#);
}
