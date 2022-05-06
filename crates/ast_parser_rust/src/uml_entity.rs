use dot::{Edge, edge, Style, Node};
use ra_ap_syntax::{ast::{self, HasName}, AstNode, match_ast};

enum DotEntity {
    Edge(Edge),
    Node(Node)
}

pub struct UMLFn {
    name: String,
    dependent_fn_names: Vec<String>,
    full_name: String
}

impl UMLFn {
    pub fn from_ast_fn(f: &ast::Fn) -> UMLFn {
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
    methods: Vec<UMLFn>,
    paths: Vec<ast::Path>
}

impl UMLClass {
    pub fn from_ast_struct(st: &ast::Struct) -> UMLClass {
        let mut st_paths = vec![];
        for node in st.syntax().descendants() {
            match_ast! {
                match node {
                    ast::Path(p) => st_paths.push(p),
                    _ => ()
                }
            }
        };
        UMLClass { name: st.name().unwrap().text().to_string(), methods: vec![] , paths: st_paths}
    }

    pub fn add_impl_fn(&mut self, f: &ast::Fn) -> () {
        let uml_fn = UMLFn::from_ast_fn(f);
        self.methods.push(uml_fn);
    }

    pub fn get_aggregation_class_name(&self) -> Vec<String> {
        self.paths
            .iter()
            .map(|p| p.to_string())
            .collect()
    }

    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        let mut label_text: Vec<&str> = vec![&self.name];
        let method_names = self.get_method_names();

        let method_names_str = method_names.join(r"/l");
        if method_names.len() > 0 {  
            label_text.insert(0, "{");
            label_text.push("|");
            label_text.push(&method_names_str);
            label_text.push("}");
        }
        let label: String = label_text.into_iter().collect();
        
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &label, Style::None, None, Some(String::from("record")))));

        // add fn's dependency
        self.get_method_dependency()
            .iter()
            .for_each(|s| dot_entities.push(DotEntity::Edge(edge(&self.name, s, "call", Style::None, None))));

        dot_entities
    }

    fn get_method_names(&self) -> Vec<String> {
        let mut names = vec![];
        self.methods
            .iter()
            .for_each(|f| {
                names.push(f.full_name.clone());
            });
        names
    }

    fn get_method_dependency(&self) -> Vec<String> {
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
    structs: Vec<(String, UMLClass)>,
    fns: Vec<UMLFn>,
    aggregations: Vec<(String, String)>
}

impl UMLModule {
    pub fn new() -> UMLModule {
        UMLModule { structs: vec![], fns: vec![], aggregations: vec![] }
    }

    pub fn add_struct(&mut self, st: UMLClass) -> () {
        let st_name = st.name.clone();

        // get aggregation class names from st
        let mut aggregation_class_relations = vec![];
        st.get_aggregation_class_name()
            .iter()
            .for_each(|s| aggregation_class_relations.push((s.clone(), st_name.clone())));
        self.aggregations.append(&mut aggregation_class_relations);

        self.structs.push((st_name.clone(), st));
    }

    pub fn add_fn(&mut self, f: UMLFn) -> () {
        self.fns.push(f);
    }

    pub fn add_ast_impl(&mut self, ip: ast::Impl) -> () {
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

    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        self.structs
            .iter()
            .for_each(|(_, st)| dot_entities.append(&mut st.get_dot_entities()));
        self.fns
            .iter()
            .for_each(|f| dot_entities.append(&mut f.get_dot_entities()));
        self.aggregations
            .iter()
            .for_each(|(from, to)| dot_entities.push(DotEntity::Edge(edge(from, to, "aggregation", Style::None, None))));
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