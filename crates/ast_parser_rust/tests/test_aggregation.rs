#![allow(dead_code)]
extern crate staticanalyzer;


#[test]
fn test_aggregation() {
    assert_eq!(
        staticanalyzer::rs2dot("tests/examples/aggregation.rs"),
r#"digraph ast {
    Amut[label="Amut"][shape="record"];
    Aconst[label="Aconst"][shape="record"];
    B[label="B"][shape="record"];
    B -> Amut[label=""][arrowhead="odiamond"];
    B -> Aconst[label=""][arrowhead="odiamond"];
}
"#
    )
}