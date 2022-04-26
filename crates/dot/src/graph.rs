use crate::{
    node::{Node},
    edge::{Edge},
    style::{Style}
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
    fn edges(&'a self) -> Edges<'a, E>;
    /// The source node for `edge`.
    fn source(&'a self, edge: &E) -> N;
    /// The target node for `edge`.
    fn target(&'a self, edge: &E) -> N;
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

// A simple wrapper around LabelledGraph that forces the labels to
// be emitted as EscStr.
pub struct LabelledGraphWithEscStrs {
    pub graph: LabelledGraph,
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

impl LabelledGraphWithEscStrs {
    pub fn new(name: &'static str,
            node_labels: Vec<Option<&'static str>>,
            edges: Vec<Edge>)
            -> LabelledGraphWithEscStrs {
        LabelledGraphWithEscStrs { graph: LabelledGraph::new(name, node_labels, edges, None) }
    }
}


impl<'a> GraphWalk<'a, Node, &'a Edge> for LabelledGraph {
    fn nodes(&'a self) -> Nodes<'a, Node> {
        (0..self.node_labels.len()).collect()
    }
    fn edges(&'a self) -> Edges<'a, &'a Edge> {
        self.edges.iter().collect()
    }
    fn source(&'a self, edge: &&'a Edge) -> Node {
        edge.from
    }
    fn target(&'a self, edge: &&'a Edge) -> Node {
        edge.to
    }
}

impl<'a> GraphWalk<'a, Node, &'a Edge> for LabelledGraphWithEscStrs {
    fn nodes(&'a self) -> Nodes<'a, Node> {
        self.graph.nodes()
    }
    fn edges(&'a self) -> Edges<'a, &'a Edge> {
        self.graph.edges()
    }
    fn source(&'a self, edge: &&'a Edge) -> Node {
        edge.from
    }
    fn target(&'a self, edge: &&'a Edge) -> Node {
        edge.to
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