/// each node is an index in a vector in the graph.
// pub type Node = usize;

use crate::{
    style::Style,
    id::{id_name, Id},
    utils::quote_string
};

pub struct Node {
    pub name: String,
    pub label: String,
    pub style: Style,
    pub color: Option<&'static str>,
    pub index: usize,
    pub shape: Option<String>
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node { name: self.name.clone(), label: self.label.clone(), style: self.style.clone(), color: self.color, index: self.index , shape: self.shape.clone()}
    }
}

impl Node {
    pub fn new(index: usize, label: &str, style: Style, color:  Option<&'static str>) -> Self {
        Node { name: id_name(&index).name().into(), label: label.to_string(), style: style, color: color, index: index, shape: None }
    }

    pub fn node_id(&self) -> &str {
        self.name.as_str()
    }

}