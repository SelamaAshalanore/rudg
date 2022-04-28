// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ! Generate files suitable for use with [Graphviz](http://www.graphviz.org/)
// !
// ! The `render` function generates output (e.g. an `output.dot` file) for
// ! use with [Graphviz](http://www.graphviz.org/) by walking a labelled
// ! graph. (Graphviz can then automatically lay out the nodes and edges
// ! of the graph, and also optionally render the graph as an image or
// ! other [output formats](
// ! http://www.graphviz.org/content/output-formats), such as SVG.)
// !
// ! Rather than impose some particular graph data structure on clients,
// ! this library exposes two traits that clients can implement on their
// ! own structs before handing them over to the rendering function.
// !
// ! Note: This library does not yet provide access to the full
// ! expressiveness of the [DOT language](
// ! http://www.graphviz.org/doc/info/lang.html). For example, there are
// ! many [attributes](http://www.graphviz.org/content/attrs) related to
// ! providing layout hints (e.g. left-to-right versus top-down, which
// ! algorithm to use, etc). The current intention of this library is to
// ! emit a human-readable .dot file with very regular structure suitable
// ! for easy post-processing.
// !
// ! # Examples
// !
// ! The first example uses a very simple graph representation: a list of
// ! pairs of ints, representing the edges (the node set is implicit).
// ! Each node label is derived directly from the int representing the node,
// ! while the edge labels are all empty strings.
// !
// ! This example also illustrates how to use `Cow<[T]>` to return
// ! an owned vector or a borrowed slice as appropriate: we construct the
// ! node vector from scratch, but borrow the edge list (rather than
// ! constructing a copy of all the edges from scratch).
// !
// ! The output from this example renders five nodes, with the first four
// ! forming a diamond-shaped acyclic graph and then pointing to the fifth
// ! which is cyclic.
// !
// ! ```rust
// ! use std::borrow::Cow;
// ! use std::io::Write;
// !
// ! type Nd = isize;
// ! type Ed = (isize,isize);
// ! struct Edges(Vec<Ed>);
// !
// ! pub fn render_to<W: Write>(output: &mut W) {
// !     let edges = Edges(vec!((0,1), (0,2), (1,3), (2,3), (3,4), (4,4)));
// !     dot::render(&edges, output).unwrap()
// ! }
// !
// ! impl<'a> dot::GraphWalk<'a, Nd, Ed> for Edges {
// !     fn nodes(&self) -> dot::Nodes<'a,Nd> {
// !         // (assumes that |N| \approxeq |E|)
// !         let &Edges(ref v) = self;
// !         let mut nodes = Vec::with_capacity(v.len());
// !         for &(s,t) in v {
// !             nodes.push(s); nodes.push(t);
// !         }
// !         nodes.sort();
// !         nodes.dedup();
// !         Cow::Owned(nodes)
// !     }
// !
// !     fn edges(&'a self) -> dot::Edges<'a,Ed> {
// !         let &Edges(ref edges) = self;
// !         Cow::Borrowed(&edges[..])
// !     }
// !
// !     fn source(&self, e: &Ed) -> Nd { e.0 }
// !
// !     fn target(&self, e: &Ed) -> Nd { e.1 }
// ! 
// !     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }
// !
// !     fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
// !         dot::Id::new(format!("N{}", *n)).unwrap()
// !     }
// ! }
// !
// ! # pub fn main() { render_to(&mut Vec::new()) }
// ! ```
// !
// ! ```no_run
// ! # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
// ! pub fn main() {
// !     use std::fs::File;
// !     let mut f = File::create("example1.dot").unwrap();
// !     render_to(&mut f)
// ! }
// ! ```
// !
// ! Output from first example (in `example1.dot`):
// !
// ! ```ignore
// ! digraph example1 {
// !     N0[label="N0"];
// !     N1[label="N1"];
// !     N2[label="N2"];
// !     N3[label="N3"];
// !     N4[label="N4"];
// !     N0 -> N1[label=""];
// !     N0 -> N2[label=""];
// !     N1 -> N3[label=""];
// !     N2 -> N3[label=""];
// !     N3 -> N4[label=""];
// !     N4 -> N4[label=""];
// ! }
// ! ```
// !
// ! The second example illustrates using `node_label` and `edge_label` to
// ! add labels to the nodes and edges in the rendered graph. The graph
// ! here carries both `nodes` (the label text to use for rendering a
// ! particular node), and `edges` (again a list of `(source,target)`
// ! indices).
// !
// ! This example also illustrates how to use a type (in this case the edge
// ! type) that shares substructure with the graph: the edge type here is a
// ! direct reference to the `(source,target)` pair stored in the graph's
// ! internal vector (rather than passing around a copy of the pair
// ! itself). Note that this implies that `fn edges(&'a self)` must
// ! construct a fresh `Vec<&'a (usize,usize)>` from the `Vec<(usize,usize)>`
// ! edges stored in `self`.
// !
// ! Since both the set of nodes and the set of edges are always
// ! constructed from scratch via iterators, we use the `collect()` method
// ! from the `Iterator` trait to collect the nodes and edges into freshly
// ! constructed growable `Vec` values (rather use the `into`
// ! from the `IntoCow` trait as was used in the first example
// ! above).
// !
// ! The output from this example renders four nodes that make up the
// ! Hasse-diagram for the subsets of the set `{x, y}`. Each edge is
// ! labelled with the &sube; character (specified using the HTML character
// ! entity `&sube`).
// !
// ! ```rust
// ! use std::io::Write;
// !
// ! type Nd = usize;
// ! type Ed<'a> = &'a (usize, usize);
// ! struct Graph { nodes: Vec<&'static str>, edges: Vec<(usize,usize)> }
// !
// ! pub fn render_to<W: Write>(output: &mut W) {
// !     let nodes = vec!("{x,y}","{x}","{y}","{}");
// !     let edges = vec!((0,1), (0,2), (1,3), (2,3));
// !     let graph = Graph { nodes: nodes, edges: edges };
// !
// !     dot::render(&graph, output).unwrap()
// ! }
// !
// ! impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph {
// !     fn nodes(&self) -> dot::Nodes<'a,Nd> { (0..self.nodes.len()).collect() }
// !     fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> { self.edges.iter().collect() }
// !     fn source(&self, e: &Ed) -> Nd { e.0 }
// !     fn target(&self, e: &Ed) -> Nd { e.1 }
// !     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example2").unwrap() }
// !     fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
// !         dot::Id::new(format!("N{}", n)).unwrap()
// !     }
// !     fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
// !         dot::LabelText::LabelStr(self.nodes[*n].into())
// !     }
// !     fn edge_label<'b>(&'b self, _: &Ed) -> dot::LabelText<'b> {
// !         dot::LabelText::LabelStr("&sube;".into())
// !     }
// ! }
// !
// ! # pub fn main() { render_to(&mut Vec::new()) }
// ! ```
// !
// ! ```no_run
// ! # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
// ! pub fn main() {
// !     use std::fs::File;
// !     let mut f = File::create("example2.dot").unwrap();
// !     render_to(&mut f)
// ! }
// ! ```
// !
// ! The third example is similar to the second, except now each node and
// ! edge now carries a reference to the string label for each node as well
// ! as that node's index. (This is another illustration of how to share
// ! structure with the graph itself, and why one might want to do so.)
// !
// ! The output from this example is the same as the second example: the
// ! Hasse-diagram for the subsets of the set `{x, y}`.
// !
// ! ```rust
// ! use std::io::Write;
// !
// ! type Nd<'a> = (usize, &'a str);
// ! type Ed<'a> = (Nd<'a>, Nd<'a>);
// ! struct Graph { nodes: Vec<&'static str>, edges: Vec<(usize,usize)> }
// !
// ! pub fn render_to<W: Write>(output: &mut W) {
// !     let nodes = vec!("{x,y}","{x}","{y}","{}");
// !     let edges = vec!((0,1), (0,2), (1,3), (2,3));
// !     let graph = Graph { nodes: nodes, edges: edges };
// !
// !     dot::render(&graph, output).unwrap()
// ! }
// !
// ! impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for Graph {
// !     fn nodes(&'a self) -> dot::Nodes<'a,Nd<'a>> {
// !         self.nodes.iter().map(|s| &s[..]).enumerate().collect()
// !     }
// !     fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> {
// !         self.edges.iter()
// !             .map(|&(i,j)|((i, &self.nodes[i][..]),
// !                           (j, &self.nodes[j][..])))
// !             .collect()
// !     }
// !     fn source(&self, e: &Ed<'a>) -> Nd<'a> { e.0 }
// !     fn target(&self, e: &Ed<'a>) -> Nd<'a> { e.1 }
// !     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example3").unwrap() }
// !     fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
// !         dot::Id::new(format!("N{}", n.0)).unwrap()
// !     }
// !     fn node_label<'b>(&'b self, n: &Nd<'b>) -> dot::LabelText<'b> {
// !         let &(i, _) = n;
// !         dot::LabelText::LabelStr(self.nodes[i].into())
// !     }
// !     fn edge_label<'b>(&'b self, _: &Ed<'b>) -> dot::LabelText<'b> {
// !         dot::LabelText::LabelStr("&sube;".into())
// !     }
// ! }
// !
// ! # pub fn main() { render_to(&mut Vec::new()) }
// ! ```
// !
// ! ```no_run
// ! # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
// ! pub fn main() {
// !     use std::fs::File;
// !     let mut f = File::create("example3.dot").unwrap();
// !     render_to(&mut f)
// ! }
// ! ```
// !
// ! # References
// !
// ! * [Graphviz](http://www.graphviz.org/)
// !
// ! * [DOT language](http://www.graphviz.org/doc/info/lang.html)

pub mod style;
pub mod arrow;
pub mod node;
pub mod edge;
pub mod graph;
pub mod id;
pub mod render;

pub use style::Style;
pub use arrow::{Arrow, ArrowShape, Side};
pub use node::{Node};
pub use edge::{edge, edge_with_arrows, Edge};
pub use graph::{GraphWalk, LabelledGraph, Nodes, Edges, Kind, DefaultStyleGraph};
pub use id::{Id, id_name};
pub use render::{render, render_opts, graph_to_string};


#[cfg(test)]
mod tests {
    use super::{LabelledGraph, edge, edge_with_arrows};
    use super::{Id, render, Style, Kind, DefaultStyleGraph};
    use super::{Arrow, ArrowShape, Side};
    use std::io;
    use std::io::prelude::*;

    fn test_input(g: LabelledGraph) -> io::Result<String> {
        let mut writer = Vec::new();
        render(&g, &mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

    // All of the tests use raw-strings as the format for the expected outputs,
    // so that you can cut-and-paste the content into a .dot file yourself to
    // see what the graphviz visualizer would produce.

    #[test]
    fn empty_graph() {
        let r = test_input(LabelledGraph::new("empty_graph", vec![], vec![], None));
        assert_eq!(r.unwrap(),
r#"digraph empty_graph {
}
"#);
    }

    #[test]
    fn single_node() {
        let r = test_input(LabelledGraph::new("single_node", vec![None], vec![], None));
        assert_eq!(r.unwrap(),
r#"digraph single_node {
    N0[label="N0"];
}
"#);
    }

    #[test]
    fn single_node_with_style() {
        let styles = Some(vec![Style::Dashed]);
        let r = test_input(LabelledGraph::new("single_node", vec![None], vec![], styles));
        assert_eq!(r.unwrap(),
r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#);
    }

    #[test]
    fn single_edge() {
        let result = test_input(LabelledGraph::new("single_edge",
                                                   vec![None, None],
                                                   vec![edge(0, 1, "E", Style::None, None)],
                                                   None));
        assert_eq!(result.unwrap(),
r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"];
}
"#);
    }

    #[test]
    fn single_edge_with_style() {
        let result = test_input(LabelledGraph::new("single_edge",
                                                   vec![None, None],
                                                   vec![edge(0, 1, "E", Style::Bold, Some("red"))],
                                                   None));
        assert_eq!(result.unwrap(),
r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"][style="bold"][color="red"];
}
"#);
    }

    #[test]
    fn test_some_labelled() {
        let styles = Some(vec![Style::None, Style::Dotted]);
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge(0, 1, "A-1", Style::None, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#);
    }

    #[test]
    fn single_cyclic_node() {
        let r = test_input(LabelledGraph::new("single_cyclic_node",
                                              vec![None],
                                              vec![edge(0, 0, "E", Style::None, None)],
                                              None));
        assert_eq!(r.unwrap(),
r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#);
    }

    #[test]
    fn hasse_diagram() {
        let r = test_input(LabelledGraph::new("hasse_diagram",
                                              vec![Some("{x,y}"), Some("{x}"), Some("{y}"), Some("{}")],
                                              vec![edge(0, 1, "", Style::None, Some("green")),
                                                   edge(0, 2, "", Style::None, Some("blue")),
                                                   edge(1, 3, "", Style::None, Some("red")),
                                                   edge(2, 3, "", Style::None, Some("black"))],
                                              None));
        assert_eq!(r.unwrap(),
r#"digraph hasse_diagram {
    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""][color="green"];
    N0 -> N2[label=""][color="blue"];
    N1 -> N3[label=""][color="red"];
    N2 -> N3[label=""][color="black"];
}
"#);
    }

    #[test]
    fn left_aligned_text() {
        let mut writer = Vec::new();

        let g = LabelledGraph::new("syntax_tree",
                                              vec!(Some(
r#"if test {
\l    branch1
\l} else {
\l    branch2
\l}
\lafterward
\l"#),
            Some("branch1"),
            Some("branch2"),
            Some("afterward")),
                                              vec![edge(0, 1, "then", Style::None, None),
                                                   edge(0, 2, "else", Style::None, None),
                                                   edge(1, 3, ";", Style::None, None),
                                                   edge(2, 3, ";", Style::None, None)],
                                                None);

        render(&g, &mut writer).unwrap();
        let mut r = String::new();
        Read::read_to_string(&mut &*writer, &mut r).unwrap();

        assert_eq!(r,
r#"digraph syntax_tree {
    N0[label="if test {
\l    branch1
\l} else {
\l    branch2
\l}
\lafterward
\l"];
    N1[label="branch1"];
    N2[label="branch2"];
    N3[label="afterward"];
    N0 -> N1[label="then"];
    N0 -> N2[label="else"];
    N1 -> N3[label=";"];
    N2 -> N3[label=";"];
}
"#);
    }

    #[test]
    fn simple_id_construction() {
        let id1 = Id::new("hello");
        match id1 {
            Ok(_) => {}
            Err(..) => panic!("'hello' is not a valid value for id anymore"),
        }
    }

    #[test]
    fn test_some_arrow() {
        let styles = Some(vec![Style::None, Style::Dotted]);
        let start  = Arrow::default();
        let end    = Arrow::from_arrow(ArrowShape::crow());
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge_with_arrows(0, 1, "A-1", Style::None, start, end, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="crow"];
}
"#);
    }

    #[test]
    fn test_some_arrows() {
        let styles = Some(vec![Style::None, Style::Dotted]);
        let start  = Arrow::from_arrow(ArrowShape::tee());
        let end    = Arrow::from_arrow(ArrowShape::Crow(Side::Left));
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge_with_arrows(0, 1, "A-1", Style::None, start, end, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="lcrow" dir="both" arrowtail="tee"];
}
"#);
    }

    #[test]
    fn invisible() {
        let r = test_input(LabelledGraph::new("single_cyclic_node",
                                              vec![None],
                                              vec![edge(0, 0, "E", Style::Invisible, None)],
                                              Some(vec![Style::Invisible])));
        assert_eq!(r.unwrap(),
                   r#"digraph single_cyclic_node {
    N0[label="N0"][style="invis"];
    N0 -> N0[label="E"][style="invis"];
}
"#);
    }

    #[test]
    fn badly_formatted_id() {
        let id2 = Id::new("Weird { struct : ure } !!!");
        match id2 {
            Ok(_) => panic!("graphviz id suddenly allows spaces, brackets and stuff"),
            Err(..) => {}
        }
    }

    fn test_input_default(g: DefaultStyleGraph) -> io::Result<String> {
        let mut writer = Vec::new();
        render(&g, &mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

    #[test]
    fn default_style_graph() {
        let r = test_input_default(
            DefaultStyleGraph::new("g", 4,
                                   vec![(0, 1), (0, 2), (1, 3), (2, 3)],
                                   Kind::Graph));
        assert_eq!(r.unwrap(),
r#"graph g {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -- N1[label=""];
    N0 -- N2[label=""];
    N1 -- N3[label=""];
    N2 -- N3[label=""];
}
"#);
    }

    #[test]
    fn default_style_digraph() {
        let r = test_input_default(
            DefaultStyleGraph::new("di", 4,
                                   vec![(0, 1), (0, 2), (1, 3), (2, 3)],
                                   Kind::Digraph));
        assert_eq!(r.unwrap(),
r#"digraph di {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
    N1 -> N3[label=""];
    N2 -> N3[label=""];
}
"#);
    }
}