use crate::{
    arrow::{Arrow},
    style::{Style}
};



pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub label: &'static str,
    pub style: Style,
    pub start_arrow: Arrow,
    pub end_arrow: Arrow,
    pub color: Option<&'static str>,
}

pub fn edge(from: usize, to: usize, label: &'static str, style: Style, color: Option<&'static str>) -> Edge {
    Edge {
        from: from,
        to: to,
        label: label,
        style: style,
        start_arrow: Arrow::default(),
        end_arrow: Arrow::default(),
        color: color,

    }
}

pub fn edge_with_arrows(from: usize, to: usize, label: &'static str, style:Style,
    start_arrow: Arrow, end_arrow: Arrow, color: Option<&'static str>) -> Edge {
    Edge {
        from: from,
        to: to,
        label: label,
        style: style,
        start_arrow: start_arrow,
        end_arrow: end_arrow,
        color: color,
    }
}
