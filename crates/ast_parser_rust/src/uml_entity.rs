use dot::{Edge, edge, Style, Node};
use ra_ap_syntax::{ast::{self, HasName}, AstNode, match_ast};
use std::collections::HashMap;

enum DotEntity {
    Edge(Edge),
    Node(Node)
}

pub struct UMLFn {
    name: String,
    dependent_fn_names: Vec<String>
}

impl UMLFn {
    pub fn from_ast_fn(f: &ast::Fn) -> UMLFn {
        let f_name = f.name().unwrap().text().to_string();


        let mut dependent_fn_names = vec![];
        // visit all Fn descendants and process CallExpr
        for node in f.syntax().descendants() {
            match_ast! {
                match node {
                    ast::CallExpr(it) => {
                        let call_name = get_call_expr_fn_names(it);
                        dependent_fn_names.push(call_name)
                    },
                    _ => (),
                }
            }
        }
        
        UMLFn { name: f_name, dependent_fn_names: dependent_fn_names }
    }

    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &self.name, Style::None, None, None)));

        self.dependent_fn_names
            .iter()
            .for_each(|f_name| dot_entities.push(DotEntity::Edge(edge(&self.name, f_name, "call", Style::None, None))));

        dot_entities
    }
}

fn get_call_expr_fn_names(call_exp: ast::CallExpr) -> String {
    let call_expr = call_exp.to_string();
    let call_names: Vec<&str> = call_expr.split("(").collect();
    String::from(call_names[0])
}

pub struct UMLClass {
    name: String,
    methods: Vec<UMLFn>
}

impl UMLClass {
    pub fn from_ast_struct(st: &ast::Struct) -> UMLClass {
        UMLClass { name: st.name().unwrap().text().to_string(), methods: vec![] }
    }

    pub fn add_impl_fn(&mut self, f: &ast::Fn) -> () {
        let uml_fn = UMLFn::from_ast_fn(f);
        self.methods.push(uml_fn);
    }

    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &self.name, Style::None, None, Some(String::from("record")))));
        self.methods
            .iter()
            .for_each(|f| {
                dot_entities.append(&mut f.get_dot_entities());
                dot_entities.push(DotEntity::Edge(edge(&f.name, &self.name,  "impl", Style::None, None)));
            });
        dot_entities
    }
}

pub struct UMLModule {
    structs: HashMap<String, UMLClass>,
    fns: Vec<UMLFn>
}

impl UMLModule {
    pub fn new() -> UMLModule {
        UMLModule { structs: HashMap::new(), fns: vec![] }
    }

    pub fn add_struct(&mut self, st: UMLClass) -> () {
        self.structs.insert(st.name.clone(), st);
    }

    pub fn add_fn(&mut self, f: UMLFn) -> () {
        self.fns.push(f);
    }

    pub fn add_ast_impl(&mut self, ip: ast::Impl) -> () {
        let struct_name: String = ip.self_ty().unwrap().to_string();
        let st = self.structs.get_mut(&struct_name).unwrap();
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

    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        self.structs
            .iter()
            .for_each(|(_, st)| dot_entities.append(&mut st.get_dot_entities()));
        self.fns
            .iter()
            .for_each(|f| dot_entities.append(&mut f.get_dot_entities()));
        dot_entities
    }

    pub fn get_node_and_edge_list(&self) -> (Vec<Node>, Vec<Edge>) {
        // transform DotEntity to nodes and edges that 'dot' can use
        // let mut label_list: Vec<&str> = vec![];
        let mut edge_list: Vec<Edge> = vec![];
        let mut node_list: Vec<Node> = vec![];
        for ent in self.get_dot_entities() {
            match ent {
                DotEntity::Edge(ent_edge) => {
                    edge_list.push(ent_edge);
                },
                DotEntity::Node(node) => {
                    node_list.push(node);
                },
            }
        }
        (node_list, edge_list)
    }
}