use super::{HasDotEntity, DotEntity};
use crate::uml_entity::*;
use dot_graph::{Node};

impl HasDotEntity for UMLClass {
    fn get_dot_entities(&self, name_prefix: &str) -> Vec<DotEntity> {
        let mut dot_entities = vec![];
        match self.kind {
            UMLClassKind::UMLClass => {
                let mut label_text: Vec<&str> = vec![&self.name];
                let method_names = self.get_method_names();
                let field_names = self.get_field_names();

                let method_names_str = method_names.join(r"\l");
                let field_names_str = field_names.join(r"\l");
                if method_names.len() + field_names.len() > 0 {
                    label_text.insert(0, "{");
                    if field_names.len() > 0 {
                        label_text.push("|");
                        label_text.push(&field_names_str);
                    } 
                    if method_names.len() > 0 {
                        label_text.push("|");
                        label_text.push(&method_names_str);    
                    }
                    label_text.push("}");
                }
                let label: String = label_text.into_iter().collect();
                
                let name = vec![name_prefix, &self.name];
                dot_entities.push(DotEntity::Node(Node::new(&name.concat()).label(&label).shape(Some("record"))));
                
            },
            UMLClassKind::UMLTrait => {
                let mut label_text: Vec<&str> = vec![r"Interface\l", &self.name];
                let method_names = self.get_method_names();

                let method_names_str = method_names.join(r"\l");
                if method_names.len() > 0 {
                    label_text.insert(0, "{");
                    label_text.push("|");
                    label_text.push(&method_names_str);
                    label_text.push("}");
                }
                let label: String = label_text.into_iter().collect();
                
                let name = vec![name_prefix, &self.name];
                dot_entities.push(DotEntity::Node(Node::new(&name.concat()).label(&label).shape(Some("record"))));
            },
        }

        dot_entities
    }
}