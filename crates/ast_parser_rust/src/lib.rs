use ra_ap_syntax::{SourceFile, Parse, ast::{self, HasModuleItem, HasName}};

pub fn parse_code(code: &str) -> Vec<String> {
    let parse: Parse<SourceFile> = SourceFile::parse(code);
    let file: SourceFile = parse.tree();

    let mut func = None;
    for item in file.items() {
        match item {
            ast::Item::Fn(f) => func = Some(f),
            _ => unreachable!(),
        }
    }
    let func_name: String = func.unwrap().name().unwrap().text().to_string();

    return vec![func_name]
}