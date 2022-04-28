use crate::{
    node::{Node},
    edge::{Edge, edge},
    style::{Style},
    id::{Id, id_name},
    arrow::Arrow,
};
use std::borrow::Cow;

pub type Nodes<'a,N> = Cow<'a,[N]>;
pub type Edges<'a,E> = Cow<'a,[E]>;

// (The type parameters in GraphWalk should be associated items,
// when/if Rust supports such.)

/// GraphWalk is an abstraction over a graph = (nodes,edges)
/// made up of node handles `N` and edge handles `E`, where each `E`
/// can be mapped to its source and target nodes.
///
/// The lifetime parameter `'a` is exposed in this trait (rather than
/// introduced as a generic parameter on each method declaration) so
/// that a client impl can choose `N` and `E` that have substructure
/// that is bound by the self lifetime `'a`.
///
/// The `nodes` and `edges` method each return instantiations of
/// `Cow<[T]>` to leave implementers the freedom to create
/// entirely new vectors or to pass back slices into internally owned
/// vectors.
pub trait GraphWalk<'a, N: Clone, E: Clone> {
    /// Returns all the nodes in this graph.
    fn nodes(&'a self) -> Nodes<'a, N>;
    /// Returns all of the edges in this graph.
    fn edges(&'a self) -> Vec<&Edge>;
    /// The source node for `edge`.
    fn source(&'a self, edge: &E) -> N;
    /// The target node for `edge`.
    fn target(&'a self, edge: &E) -> N;

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
    fn node_shape(&'a self, _node: &N) -> Option<String> {
        None
    }

    /// Maps `n` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is just the output from `node_id`.
    fn node_label(&'a self, n: &N) -> String {
        self.node_id(n).name().to_string()
    }

    /// Maps `e` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is in fact the empty string.
    fn edge_label(&'a self, e: &E) -> String {
        let _ignored = e;
        "".into()
    }

    /// Maps `n` to a style that will be used in the rendered output.
    fn node_style(&'a self, _n: &N) -> Style {
        Style::None
    }

    /// Maps `n` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    ///
    /// [1]: https://graphviz.gitlab.io/_pages/doc/info/colors.html
    fn node_color(&'a self, _node: &N) -> Option<String> {
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
    fn edge_color(&'a self, _e: &E) -> Option<String> {
        None
    }

    /// The kind of graph, defaults to `Kind::Digraph`.
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Digraph
    }
}



pub struct LabelledGraph {
    /// The name for this graph. Used for labelling generated `digraph`.
    pub name: &'static str,

    /// Each node is an index into `node_labels`; these labels are
    /// used as the label text for each node. (The node *names*,
    /// which are unique identifiers, are derived from their index
    /// in this array.)
    ///
    /// If a node maps to None here, then just use its name as its
    /// text.
    pub node_labels: Vec<Option<&'static str>>,

    pub node_styles: Vec<Style>,

    /// Each edge relates a from-index to a to-index along with a
    /// label; `edges` collects them.
    edges: Vec<Edge>,
}

impl LabelledGraph {
    pub fn new(name: &'static str,
            node_labels: Vec<Option<&'static str>>,
            edges: Vec<Edge>,
            node_styles: Option<Vec<Style>>)
            -> LabelledGraph {
        let count = node_labels.len();
        LabelledGraph {
            name: name,
            node_labels: node_labels.into_iter().collect(),
            edges: edges,
            node_styles: match node_styles {
                Some(nodes) => nodes,
                None => vec![Style::None; count],
            },
        }
    }
}

impl<'a> GraphWalk<'a, Node, &'a Edge> for LabelledGraph {
    fn nodes(&'a self) -> Nodes<'a, Node> {
        (0..self.node_labels.len()).collect()
    }
    fn edges(&'a self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }
    fn source(&'a self, edge: &&'a Edge) -> Node {
        edge.from
    }
    fn target(&'a self, edge: &&'a Edge) -> Node {
        edge.to
    }

    fn graph_id(&'a self) -> Id<'a> {
        Id::new(&self.name[..]).unwrap()
    }
    fn node_id(&'a self, n: &Node) -> Id<'a> {
        id_name(n)
    }
    fn node_label(&'a self, n: &Node) -> String {
        match self.node_labels[*n] {
            Some(ref l) => (*l).into(),
            None => id_name(n).name().to_string(),
        }
    }
    fn edge_label(&'a self, e: &&'a Edge) -> String {
        e.label.into()
    }
    fn node_style(&'a self, n: &Node) -> Style {
        self.node_styles[*n]
    }
    fn edge_style(&'a self, e: &&'a Edge) -> Style {
        e.style
    }
    fn edge_color(&'a self, e: &&'a Edge) -> Option<String>
    {
        match e.color {
            Some(l) => {
                Some((*l).into())
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

/// Graph kind determines if `digraph` or `graph` is used as keyword
/// for the graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Digraph,
    Graph,
}

impl Kind {
    /// The keyword to use to introduce the graph.
    /// Determines which edge syntax must be used, and default style.
    pub fn keyword(&self) -> &'static str {
        match *self {
            Kind::Digraph => "digraph",
            Kind::Graph => "graph"
        }
    }

    /// The edgeop syntax to use for this graph kind.
    pub fn edgeop(&self) -> &'static str {
        match *self {
            Kind::Digraph => "->",
            Kind::Graph => "--",
        }
    }
}

pub type SimpleEdge = (Node, Node);

pub struct DefaultStyleGraph {
    /// The name for this graph. Used for labelling generated graph
    name: &'static str,
    nodes: usize,
    edges: Vec<Edge>,
    kind: Kind,
}

impl DefaultStyleGraph {
    pub fn new(name: &'static str,
            nodes: usize,
            edges: Vec<SimpleEdge>,
            kind: Kind)
            -> DefaultStyleGraph {
        assert!(!name.is_empty());
        let mut results: Vec<Edge> = vec![];
        for (start, end) in edges.iter() {
            let edge = edge(*start, *end, "", Style::None, None);
            results.push(edge);
        }
        DefaultStyleGraph {
            name: name,
            nodes: nodes,
            edges: results,
            kind: kind,
        }
    }
}

impl<'a> GraphWalk<'a, Node, &'a SimpleEdge> for DefaultStyleGraph {
    fn nodes(&'a self) -> Nodes<'a, Node> {
        (0..self.nodes).collect()
    }
    fn edges(&'a self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }
    fn source(&'a self, edge: &&'a SimpleEdge) -> Node {
        edge.0
    }
    fn target(&'a self, edge: &&'a SimpleEdge) -> Node {
        edge.1
    }

    fn graph_id(&'a self) -> Id<'a> {
        Id::new(&self.name[..]).unwrap()
    }
    fn node_id(&'a self, n: &Node) -> Id<'a> {
        id_name(n)
    }
    fn kind(&self) -> Kind {
        self.kind
    }
}