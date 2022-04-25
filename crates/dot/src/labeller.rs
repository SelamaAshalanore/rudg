use crate::{
    id::{Id, id_name},
    label_text::{LabelText, LabelText::*},
    style::{Style},
    arrow::Arrow,
    graph::{Kind, LabelledGraph, LabelledGraphWithEscStrs},
    node::{Node},
    edge::{Edge}
};


/// The graph instance is responsible for providing the DOT compatible
/// identifiers for the nodes and (optionally) rendered labels for the nodes and
/// edges, as well as an identifier for the graph itself.
pub trait Labeller<'a,N,E> {
    /// Must return a DOT compatible identifier naming the graph.
    fn graph_id(&'a self) -> Id<'a>;

    /// Maps `n` to a unique identifier with respect to `self`. The
    /// implementer is responsible for ensuring that the returned name
    /// is a valid DOT identifier.
    fn node_id(&'a self, n: &N) -> Id<'a>;

    /// Maps `n` to one of the [graphviz `shape` names][1]. If `None`
    /// is returned, no `shape` attribute is specified.
    ///
    /// [1]: http://www.graphviz.org/content/node-shapes
    fn node_shape(&'a self, _node: &N) -> Option<LabelText<'a>> {
        None
    }

    /// Maps `n` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is just the output from `node_id`.
    fn node_label(&'a self, n: &N) -> LabelText<'a> {
        LabelStr(self.node_id(n).name())
    }

    /// Maps `e` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is in fact the empty string.
    fn edge_label(&'a self, e: &E) -> LabelText<'a> {
        let _ignored = e;
        LabelStr("".into())
    }

    /// Maps `n` to a style that will be used in the rendered output.
    fn node_style(&'a self, _n: &N) -> Style {
        Style::None
    }

    /// Maps `n` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    ///
    /// [1]: https://graphviz.gitlab.io/_pages/doc/info/colors.html
    fn node_color(&'a self, _node: &N) -> Option<LabelText<'a>> {
        None
    }

    /// Maps `e` to arrow style that will be used on the end of an edge.
    /// Defaults to default arrow style.
    fn edge_end_arrow(&'a self, _e: &E) -> Arrow {
        Arrow::default()
    }

    /// Maps `e` to arrow style that will be used on the end of an edge.
    /// Defaults to default arrow style.
    fn edge_start_arrow(&'a self, _e: &E) -> Arrow {
        Arrow::default()
    }

    /// Maps `e` to a style that will be used in the rendered output.
    fn edge_style(&'a self, _e: &E) -> Style {
        Style::None
    }

    /// Maps `e` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    ///
    /// [1]: https://graphviz.gitlab.io/_pages/doc/info/colors.html
    fn edge_color(&'a self, _e: &E) -> Option<LabelText<'a>> {
        None
    }

    /// The kind of graph, defaults to `Kind::Digraph`.
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Digraph
    }
}


impl<'a> Labeller<'a, Node, &'a Edge> for LabelledGraph {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new(&self.name[..]).unwrap()
    }
    fn node_id(&'a self, n: &Node) -> Id<'a> {
        id_name(n)
    }
    fn node_label(&'a self, n: &Node) -> LabelText<'a> {
        match self.node_labels[*n] {
            Some(ref l) => LabelStr((*l).into()),
            None => LabelStr(id_name(n).name()),
        }
    }
    fn edge_label(&'a self, e: &&'a Edge) -> LabelText<'a> {
        LabelStr(e.label.into())
    }
    fn node_style(&'a self, n: &Node) -> Style {
        self.node_styles[*n]
    }
    fn edge_style(&'a self, e: &&'a Edge) -> Style {
        e.style
    }
    fn edge_color(&'a self, e: &&'a Edge) -> Option<LabelText<'a>>
    {
        match e.color {
            Some(l) => {
                Some(LabelStr((*l).into()))
            },
            None => None,
        }
    }
    fn edge_end_arrow(&'a self, e: &&'a Edge) -> Arrow {
        e.end_arrow.clone()
    }

    fn edge_start_arrow(&'a self, e: &&'a Edge) -> Arrow {
        e.start_arrow.clone()
    }
}

impl<'a> Labeller<'a, Node, &'a Edge> for LabelledGraphWithEscStrs {
    fn graph_id(&'a self) -> Id<'a> {
        self.graph.graph_id()
    }
    fn node_id(&'a self, n: &Node) -> Id<'a> {
        self.graph.node_id(n)
    }
    fn node_label(&'a self, n: &Node) -> LabelText<'a> {
        match self.graph.node_label(n) {
            LabelStr(s) | EscStr(s) | HtmlStr(s) => EscStr(s),
        }
    }
    fn node_color(&'a self, n: &Node) -> Option<LabelText<'a>> {
        match self.graph.node_color(n) {
            Some(LabelStr(s)) | Some(EscStr(s)) | Some(HtmlStr(s)) => Some(EscStr(s)),
            None => None,
        }
    }
    fn edge_label(&'a self, e: &&'a Edge) -> LabelText<'a> {
        match self.graph.edge_label(e) {
            LabelStr(s) | EscStr(s) | HtmlStr(s) => EscStr(s),
        }
    }
    fn edge_color(&'a self, e: &&'a Edge) -> Option<LabelText<'a>> {
        match self.graph.edge_color(e) {
            Some(LabelStr(s)) | Some(EscStr(s)) | Some(HtmlStr(s)) => Some(EscStr(s)),
            None => None,
        }
    }
}