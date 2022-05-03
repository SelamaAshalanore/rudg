use crate::{
    node::{Node},
    edge::{Edge, edge},
    style::{Style},
    id::{Id, id_name},
};
use std::borrow::Cow;

pub type Nodes<'a,N> = Cow<'a,[N]>;

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
pub trait GraphWalk<'a, N: Clone> {
    /// Returns all the nodes in this graph.
    fn nodes(&'a self) -> Vec<&Node>;
    /// Returns all of the edges in this graph.
    fn edges(&'a self) -> Vec<&Edge>;

    /// Must return a DOT compatible identifier naming the graph.
    fn graph_id(&'a self) -> Id<'a>;

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
    nodes: Vec<Node>
}

impl LabelledGraph {
    pub fn new(name: &'static str,
            node_labels: Vec<Option<&'static str>>,
            edges: Vec<Edge>,
            node_styles: Option<Vec<Style>>)
            -> LabelledGraph {
        let count = node_labels.len();
        let mut nodes: Vec<Node> = vec![];
        for i in 0..count {
            let node_label = match node_labels[i] {
                Some(ref l) => (*l).into(),
                None => id_name(&i).name().to_string(),
            };
            let node_style = match node_styles {
                Some(ref styles) => styles[i],
                None => Style::None,
            };
            let node: Node = Node::new(i, &node_label, node_style, None);
            nodes.push(node);
        };
        LabelledGraph {
            name: name,
            node_labels: node_labels.into_iter().collect(),
            edges: edges,
            node_styles: match node_styles {
                Some(nodes) => nodes,
                None => vec![Style::None; count],
            },
            nodes: nodes
        }
    }
}

impl<'a> GraphWalk<'a, Node> for LabelledGraph {
    fn nodes(&'a self) -> Vec<&Node> {
        self.nodes.iter().map(|node| node).collect()
    }
    fn edges(&'a self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }

    fn graph_id(&'a self) -> Id<'a> {
        Id::new(&self.name[..]).unwrap()
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

pub type SimpleEdge = (usize, usize);

pub struct DefaultStyleGraph {
    /// The name for this graph. Used for labelling generated graph
    name: &'static str,
    edges: Vec<Edge>,
    kind: Kind,
    node_vec: Vec<Node>
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
            let edge = edge(id_name(start).as_slice(), id_name(end).as_slice(), "", Style::None, None);
            results.push(edge);
        }
        DefaultStyleGraph {
            name: name,
            edges: results,
            kind: kind,
            node_vec: (0..nodes).map(|index| Node::new(index, id_name(&index).as_slice(), Style::None, None)).collect()
        }
    }
}

impl<'a> GraphWalk<'a, Node> for DefaultStyleGraph {
    fn nodes(&'a self) -> Vec<&Node> {
        self.node_vec.iter().collect()
    }
    fn edges(&'a self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }

    fn graph_id(&'a self) -> Id<'a> {
        Id::new(&self.name[..]).unwrap()
    }
    fn kind(&self) -> Kind {
        self.kind
    }
}