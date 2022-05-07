use ra_ap_syntax::{ast::{self, HasName, HasModuleItem, Path}, AstNode, match_ast, SourceFile};

use crate::ast_parser::{HasUMLFn, HasUMLClass, HasUMLAggregation};

pub struct UMLAggregation {
    pub from: String,
    pub to: String
}


impl UMLAggregation {
    pub fn new(from: &str, to: &str) -> UMLAggregation {
        UMLAggregation { from: String::from(from), to: String::from(to) }
    }
}

pub struct UMLDependency {
    pub from: String,
    pub to: String
}


impl UMLDependency {
    pub fn new(from: &str, to: &str) -> UMLDependency {
        UMLDependency { from: String::from(from), to: String::from(to) }
    }
}

pub struct UMLFn {
    pub name: String,
    pub dependent_fn_names: Vec<String>,
    full_name: String
}

impl UMLFn {
    pub fn new(name: &str, dependent_fn_names: Vec<String>, full_name: &str) -> UMLFn {
        UMLFn { name: String::from(name), dependent_fn_names: dependent_fn_names, full_name: String::from(full_name) }
    }
}

pub struct UMLClass {
    pub name: String,
    methods: Vec<UMLFn>,
    method_names: Vec<String>,
    fields: Vec<String>,
    aggregations_list: Vec<String>,
    dependency_list: Vec<String>
}


impl UMLClass {
    pub fn new(name: &str, fields: Vec<String>, method_names: Vec<String>) -> UMLClass {
        UMLClass { name: String::from(name), methods: vec![], fields: fields, aggregations_list: vec![], dependency_list: vec![], method_names: method_names}
    }

    pub fn merge_from(&mut self, from: &mut UMLClass) -> () {
        if self.name != from.name {()}
        println!("struct from has {} methods", from.methods.len());

        self.methods.append(&mut from.methods);
        println!("now {} has {} methods", self.name, self.methods.len());
        self.dependency_list.append(&mut from.dependency_list);
    }

    fn add_impl_fn(&mut self, f: &ast::Fn) -> () {
        self.methods.append(&mut f.get_uml_fn());
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
        self.fields.clone()
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
                    self.add_structs(ip.get_uml_class());
                },
                ast::Item::Struct(st) => {
                    self.add_structs(st.get_uml_class());
                    self.add_aggregations(&mut st.get_uml_aggregation());
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

    fn add_structs(&mut self, st_list: Vec<UMLClass>) -> () {
        for mut st in st_list {
            if self.get_struct_names().contains(&st.name) {
                println!("{} struct exists!", &st.name);
                self.get_mut_struct(&st.name).unwrap().merge_from(&mut st);
            } else {
                self.add_struct(st)
            }
        }
    }

    fn add_fns(&mut self, fns: &mut Vec<UMLFn>) -> () {
        self.fns.append(fns)
    }

    fn add_aggregations(&mut self, aggregations: &mut Vec<UMLAggregation>) -> () {
        self.aggregations.append(aggregations);
    }

    fn add_ast_impl(&mut self, ip: ast::Impl) -> () {
        let struct_name: String = ip.self_ty().unwrap().to_string();
        let st = self.get_mut_struct(&struct_name).unwrap();
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

    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        match self.structs.iter_mut().find(|(st_name, _)| st_name == struct_name) {
            Some((_, c)) => Some(c),
            None => None
        }
    }

    fn get_struct_names(&self) -> Vec<String> {
        self.structs
            .iter()
            .map(|(st_name, _)| st_name.clone())
            .collect()
    }
}