/// each node is an index in a vector in the graph.
pub type Node = usize;

pub enum NodeLabels<L> {
    AllNodesLabelled(Vec<L>),
    UnlabelledNodes(usize),
    SomeNodesLabelled(Vec<Option<L>>),
}

pub type Trivial = NodeLabels<&'static str>;

impl NodeLabels<&'static str> {
    pub fn into_opt_strs(self) -> Vec<Option<&'static str>> {
        match self {
            NodeLabels::UnlabelledNodes(len) => vec![None; len],
            NodeLabels::AllNodesLabelled(lbls) => lbls.into_iter().map(|l| Some(l)).collect(),
            NodeLabels::SomeNodesLabelled(lbls) => lbls.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &NodeLabels::UnlabelledNodes(len) => len,
            &NodeLabels::AllNodesLabelled(ref lbls) => lbls.len(),
            &NodeLabels::SomeNodesLabelled(ref lbls) => lbls.len(),
        }
    }
}