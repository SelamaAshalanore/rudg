#![allow(dead_code)]
extern crate staticanalyzer;


// #[test]
// fn test_aggregation() {
//     assert_eq!(
//         staticanalyzer::rs2dot("tests/examples/association.rs"),
// r#"digraph ast {
//     A[label="{A|b() -> B}"][shape="record"];
//     Ab[label="{Ab|b() -> B}"][shape="record"];
//     B[label="{B|a() -> Ab}"][shape="record"];
//     Ab -> B[label=""][arrowhead="none"];
//     B -> A[label=""][arrowhead="vee"];
// }
// "#
//     )
// }