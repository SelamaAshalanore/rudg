/// each node is an index in a vector in the graph.
// pub type Node = usize;

use crate::{
    style::Style,
    id::{id_name, Id},
    utils::quote_string,
    render::{RenderOption}
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

    pub fn to_dot_string(&self, options: &[RenderOption]) -> String {
        let colorstring: String;

        let escaped: String = quote_string(self.label.clone());
        let shape: String;

        let mut text = vec![self.node_id()];

        if !options.contains(&RenderOption::NoNodeLabels) {
            text.push("[label=");
            text.push(escaped.as_str());
            text.push("]");
        }

        let style = self.style;
        if !options.contains(&RenderOption::NoNodeStyles) && style != Style::None {
            text.push("[style=\"");
            text.push(style.as_slice());
            text.push("\"]");
        }

        let color = self.color;
        if !options.contains(&RenderOption::NoNodeColors) {
            if let Some(c) = color {
                colorstring = quote_string(c.to_string());
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if let Some(s) = self.shape.clone() {
            shape = s;
            text.push("[shape=");
            text.push(&shape);
            text.push("]");
        }

        text.push(";");
        return text.into_iter().collect();
    }

}