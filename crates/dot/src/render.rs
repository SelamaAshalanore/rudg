use std::io::prelude::*;
use std::io;

use crate::{
    graph::{GraphWalk, LabelledGraph},
    style::{Style},
    utils::{quote_string}, edge::EdgeTrait
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RenderOption {
    NoEdgeLabels,
    NoNodeLabels,
    NoEdgeStyles,
    NoEdgeColors,
    NoNodeStyles,
    NoNodeColors,
    NoArrows,
}

/// Returns vec holding all the default render options.
pub fn default_options() -> Vec<RenderOption> {
    vec![]
}

/// Renders graph `g` into the writer `w` in DOT syntax.
/// (Simple wrapper around `render_opts` that passes a default set of options.)
pub fn render<'a,
              N: Clone + 'a,
              G: GraphWalk<'a, N>,
              W: Write>
    (g: &'a G,
     w: &mut W)
     -> io::Result<()> {
    render_opts(g, w, &[])
}

/// Renders graph `g` into the writer `w` in DOT syntax.
/// (Main entry point for the library.)
pub fn render_opts<'a,
                   N: Clone + 'a,
                   G: GraphWalk<'a, N>,
                   W: Write>
    (g: &'a G,
     w: &mut W,
     options: &[RenderOption])
     -> io::Result<()> {
    fn writeln<W: Write>(w: &mut W, arg: &[&str]) -> io::Result<()> {
        for &s in arg {
            w.write_all(s.as_bytes())?;
        }
        write!(w, "\n")
    }

    fn indent<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(b"    ")
    }

    writeln(w, &[g.kind().keyword(), " ", g.graph_id().as_slice(), " {"])?;
    for n in g.nodes().iter() {
        let colorstring: String;

        indent(w)?;
        let id = g.node_id(n);

        let escaped: String = quote_string(g.node_label(n));
        let shape: String;

        let mut text = vec![id.as_slice()];

        if !options.contains(&RenderOption::NoNodeLabels) {
            text.push("[label=");
            text.push(escaped.as_str());
            text.push("]");
        }

        let style = g.node_style(n);
        if !options.contains(&RenderOption::NoNodeStyles) && style != Style::None {
            text.push("[style=\"");
            text.push(style.as_slice());
            text.push("\"]");
        }

        let color = g.node_color(n);
        if !options.contains(&RenderOption::NoNodeColors) {
            if let Some(c) = color {
                colorstring = quote_string(c);
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if let Some(s) = g.node_shape(n) {
            shape = s;
            text.push("[shape=");
            text.push(&shape);
            text.push("]");
        }

        text.push(";");
        writeln(w, &text)?;
    }

    let edge_symbol = g.kind().edgeop();
    for e in g.edges().iter() {
        indent(w)?;
        let mut text: Vec<&str> = vec![];
        let edge_dot_string: String = e.to_dot_string(edge_symbol, options);
        text.push(&edge_dot_string.as_str());
        writeln(w, &text)?;
    }

    writeln(w, &["}"])
}

pub fn graph_to_string(g: LabelledGraph) -> io::Result<String> {
    let mut writer = Vec::new();
    render(&g, &mut writer).unwrap();
    let mut s = String::new();
    Read::read_to_string(&mut &*writer, &mut s)?;
    Ok(s)
}