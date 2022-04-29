/// each node is an index in a vector in the graph.
// pub type Node = usize;

use crate::{
    style::Style,
    id::id_name
};

pub struct Node {
    pub name: String,
    pub label: &'static str,
    pub style: Style,
    pub color: Option<&'static str>,
    pub index: usize
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node { name: self.name.clone(), label: self.label, style: self.style.clone(), color: self.color, index: self.index }
    }
}

impl Node {
    pub fn new(index: usize) -> Self {
        Node { name: id_name(&index).name().into(), label: "", style: Style::None, color: None, index: index }
    }
}