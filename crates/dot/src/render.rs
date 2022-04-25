use std::io::prelude::*;
use std::io;

use crate::{
    labeller::{Labeller},
    graph::{GraphWalk, LabelledGraph},
    style::{Style}
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
              E: Clone + 'a,
              G: Labeller<'a, N, E> + GraphWalk<'a, N, E>,
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
                   E: Clone + 'a,
                   G: Labeller<'a, N, E> + GraphWalk<'a, N, E>,
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
        let colorstring;

        indent(w)?;
        let id = g.node_id(n);

        let escaped = &g.node_label(n).to_dot_string();
        let shape;

        let mut text = vec![id.as_slice()];

        if !options.contains(&RenderOption::NoNodeLabels) {
            text.push("[label=");
            text.push(escaped);
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
                colorstring = c.to_dot_string();
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if let Some(s) = g.node_shape(n) {
            shape = s.to_dot_string();
            text.push("[shape=");
            text.push(&shape);
            text.push("]");
        }

        text.push(";");
        writeln(w, &text)?;
    }

    for e in g.edges().iter() {
        let colorstring;
        let escaped_label = &g.edge_label(e).to_dot_string();
        let start_arrow = g.edge_start_arrow(e);
        let end_arrow = g.edge_end_arrow(e);
        let start_arrow_s = start_arrow.to_dot_string();
        let end_arrow_s = end_arrow.to_dot_string();

        indent(w)?;
        let source = g.source(e);
        let target = g.target(e);
        let source_id = g.node_id(&source);
        let target_id = g.node_id(&target);

        let mut text = vec![source_id.as_slice(), " ",
                            g.kind().edgeop(), " ",
                            target_id.as_slice()];

        if !options.contains(&RenderOption::NoEdgeLabels) {
            text.push("[label=");
            text.push(escaped_label);
            text.push("]");
        }

        let style = g.edge_style(e);
        if !options.contains(&RenderOption::NoEdgeStyles) && style != Style::None {
            text.push("[style=\"");
            text.push(style.as_slice());
            text.push("\"]");
        }

        let color = g.edge_color(e);
        if !options.contains(&RenderOption::NoEdgeColors) {
            if let Some(c) = color {
                colorstring = c.to_dot_string();
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if !options.contains(&RenderOption::NoArrows) &&
            (!start_arrow.is_default() || !end_arrow.is_default()) {
            text.push("[");
            if !end_arrow.is_default() {
                text.push("arrowhead=\"");
                text.push(&end_arrow_s);
                text.push("\"");
            }
            if !start_arrow.is_default() {
                text.push(" dir=\"both\" arrowtail=\"");
                text.push(&start_arrow_s);
                text.push("\"");
            }

            text.push("]");
        }

        text.push(";");
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