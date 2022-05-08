
use dot::{Edge, edge, Style, Node, edge_with_arrows, Arrow, ArrowShape, Fill, Side};
use crate::uml_entity::*;
pub enum DotEntity {
    Edge(Edge),
    Node(Node)
}

pub trait UMLEntity {
    fn get_dot_entities(&self) -> Vec<DotEntity>;
}


impl UMLEntity for UMLAggregation {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        vec![DotEntity::Edge(edge_with_arrows(
            &self.from, 
            &self.to, 
            "", 
            Style::None, 
            Arrow::default(),
            Arrow::from_arrow(ArrowShape::Diamond(Fill::Open, Side::Both)),
            None
        ))]
    }
}

impl UMLEntity for UMLDependency {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        vec![DotEntity::Edge(edge(
            &self.from, 
            &self.to, 
            "call", 
            Style::None,
            None
        ))]
    }
}

impl UMLEntity for UMLFn {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &self.name, Style::None, None, None)));

        self.dependent_fn_names
            .iter()
            .for_each(|f_name| dot_entities.push(DotEntity::Edge(edge(&self.name, f_name, "call", Style::None, None))));

        dot_entities
    }
}


impl UMLEntity for UMLClass {
    fn get_dot_entities(&self) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        let mut label_text: Vec<&str> = vec![&self.name];
        let method_names = self.get_method_names();
        let field_names = self.get_field_names();

        let method_names_str = method_names.join(r"/l");
        let field_names_str = field_names.join(r"/l");
        if method_names.len() + field_names.len() > 0 {
            label_text.insert(0, "{");
            if field_names.len() > 0 {
                label_text.push("|");
                label_text.push(&field_names_str);
            } else if method_names.len() > 0 {
                label_text.push("|");
                label_text.push(&method_names_str);    
            }
            label_text.push("}");
        }
        let label: String = label_text.into_iter().collect();
        
        dot_entities.push(DotEntity::Node(Node::new(&self.name, &label, Style::None, None, Some(String::from("record")))));

        dot_entities
    }
}


impl UMLEntity for UMLModule {
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
            .for_each(|ag| dot_entities.append(&mut ag.get_dot_entities()));
        self.dependency
            .iter()
            .for_each(|ag| dot_entities.append(&mut ag.get_dot_entities()));
        dot_entities
    }
}