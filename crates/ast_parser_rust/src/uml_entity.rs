use ra_ap_syntax::{ast::{self, HasName, HasModuleItem}, AstNode, match_ast, SourceFile};

use crate::ast_parser::HasUMLFn;

pub struct UMLAggregation {
    pub from: String,
    pub to: String
}


impl UMLAggregation {
    fn new(from: &str, to: &str) -> UMLAggregation {
        UMLAggregation { from: String::from(from), to: String::from(to) }
    }
}

pub struct UMLFn {
    pub name: String,
    pub dependent_fn_names: Vec<String>,
    pub full_name: String
}


impl UMLFn {
    fn from_ast_fn(f: &ast::Fn) -> UMLFn {
        let f_name = f.name().unwrap().text().to_string();
        let mut full_name: String = f_name.clone();

        let mut dependent_fn_names = vec![];
        // visit all Fn descendants and process CallExpr
        for node in f.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(call_name)
                    },
                    ast::ParamList(pl) => {
                        full_name.push_str(&pl.to_string());
                    },
                    _ => {
                        // println!("{:?}", node);
                        // println!("{}", node)
                    },
                }
            }
        }
        
        UMLFn { name: f_name, dependent_fn_names: dependent_fn_names, full_name: full_name}
    }
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}

pub struct UMLClass {
    pub name: String,
    pub methods: Vec<UMLFn>,
    pub paths: Vec<ast::Path>,
    pub record_fields: Vec<ast::RecordField>,
    aggregations_list: Vec<String>,
    compositions_list: Vec<String>,
}


impl UMLClass {
    fn from_ast_struct(st: &ast::Struct) -> UMLClass {
        let mut st_paths = vec![];
        let mut record_fields = vec![];
        for node in st.syntax().descendants() {
            match_ast! {
                match node {
                    ast::Path(p) => st_paths.push(p),
                    ast::RecordField(rf) => {
                        record_fields.push(rf)
                    },
                    _ => ()
                }
            }
            // println!("{:?}", node);
            // println!("{}", node);
        };
        UMLClass { name: st.name().unwrap().text().to_string(), methods: vec![] , paths: st_paths, record_fields: record_fields, aggregations_list: vec![], compositions_list: vec![]}
    }

    fn add_impl_fn(&mut self, f: &ast::Fn) -> () {
        let uml_fn = UMLFn::from_ast_fn(f);
        self.methods.push(uml_fn);
    }

    fn get_aggregation_class_name(&self) -> Vec<String> {
        self.aggregations_list.clone()
    }

    pub fn get_method_names(&self) -> Vec<String> {
        let mut names = vec![];
        self.methods
            .iter()
            .for_each(|f| {
                names.push(f.full_name.clone());
            });
        names
    }

    pub fn get_field_names(&self) -> Vec<String> {
        let mut names = vec![];
        self.record_fields
            .iter()
            .for_each(|p| {
                names.push(p.to_string())
            });
        names
    }

    pub fn get_method_dependency(&self) -> Vec<String> {
        let mut dep_names = vec![];
        self.methods
            .iter()
            .for_each(|f| {
                dep_names.extend(f.dependent_fn_names.iter().map(|s| s.clone()));
            });
            dep_names
    }
}

pub struct UMLModule {
    pub structs: Vec<(String, UMLClass)>,
    pub fns: Vec<UMLFn>,
    pub aggregations: Vec<UMLAggregation>
}

impl UMLModule {
    pub fn new() -> UMLModule {
        UMLModule { structs: vec![], fns: vec![], aggregations: vec![] }
    }

    pub fn parse_source_file(&mut self, src_file: SourceFile) -> () {
        // visit all items in SourceFile and extract dot entities from every type of them
        for item in src_file.items() {
            match item {
                ast::Item::Fn(f) => {
                    self.add_fns(&mut f.get_uml_fn());
                },
                ast::Item::Impl(ip) => {
                    self.add_ast_impl(ip);
                },
                ast::Item::Struct(st) => {
                    let uml_class = UMLClass::from_ast_struct(&st);
                    self.add_struct(uml_class);
                },
                _ => (),
            }
        }
    }

    fn add_struct(&mut self, st: UMLClass) -> () {
        let st_name = st.name.clone();

        // get aggregation class names from st
        let mut aggregation_class_relations: Vec<UMLAggregation> = vec![];
        st.get_aggregation_class_name()
            .iter()
            .for_each(|s| aggregation_class_relations.push(UMLAggregation::new(s, &st_name)));
        self.aggregations.append(&mut aggregation_class_relations);

        self.structs.push((st_name.clone(), st));
    }

    fn add_fns(&mut self, fns: &mut Vec<UMLFn>) -> () {
        self.fns.append(fns)
    }

    fn add_ast_impl(&mut self, ip: ast::Impl) -> () {
        let struct_name: String = ip.self_ty().unwrap().to_string();
        let st = self.get_mut_struct(&struct_name);
        let impl_funcs = ip.get_or_create_assoc_item_list().assoc_items();
        for impl_func in impl_funcs {
            match impl_func {
                ast::AssocItem::Fn(f) => {
                    st.add_impl_fn(&f)    
                },
                _ => ()
            }
        }
    }

    fn get_mut_struct(&mut self, struct_name: &str) -> &mut UMLClass {
        let (_, st) = self.structs.iter_mut().find(|(st_name, _)| st_name == struct_name).unwrap();
        st
    }
}