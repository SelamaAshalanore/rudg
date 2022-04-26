
// There is a tension in the design of the labelling API.
//
// For example, I considered making a `Labeller<T>` trait that
// provides labels for `T`, and then making the graph type `G`
// implement `Labeller<Node>` and `Labeller<Edge>`. However, this is
// not possible without functional dependencies. (One could work
// around that, but I did not explore that avenue heavily.)
//
// Another approach that I actually used for a while was to make a
// `Label<Context>` trait that is implemented by the client-specific
// Node and Edge types (as well as an implementation on Graph itself
// for the overall name for the graph). The main disadvantage of this
// second approach (compared to having the `G` type parameter
// implement a Labelling service) that I have encountered is that it
// makes it impossible to use types outside of the current crate
// directly as Nodes/Edges; you need to wrap them in newtype'd
// structs. See e.g. the `No` and `Ed` structs in the examples. (In
// practice clients using a graph in some other crate would need to
// provide some sort of adapter shim over the graph anyway to
// interface with this library).
//
// Another approach would be to make a single `Labeller<N,E>` trait
// that provides three methods (graph_label, node_label, edge_label),
// and then make `G` implement `Labeller<N,E>`. At first this did not
// appeal to me, since I had thought I would need separate methods on
// each data variant for dot-internal identifiers versus user-visible
// labels. However, the identifier/label distinction only arises for
// nodes; graphs themselves only have identifiers, and edges only have
// labels.
//
// So in the end I decided to use the third approach described above.

/// Each instance of a type that implements `Label<C>` maps to a
/// unique identifier with respect to `C`, which is used to identify
/// it in the generated .dot file. They can also provide more
/// elaborate (and non-unique) label text that is used in the graphviz
/// rendered output.

/// The text for a graphviz label on a node or edge.
use std::borrow::Cow;
use self::LabelText::*;
pub enum LabelText<'a> {
    /// This kind of label preserves the text directly as is.
    ///
    /// Occurrences of backslashes (`\`) are escaped, and thus appear
    /// as backslashes in the rendered label.
    LabelStr(Cow<'a, str>),

    /// This kind of label uses the graphviz label escString type:
    /// http://www.graphviz.org/content/attrs#kescString
    ///
    /// Occurrences of backslashes (`\`) are not escaped; instead they
    /// are interpreted as initiating an escString escape sequence.
    ///
    /// Escape sequences of particular interest: in addition to `\n`
    /// to break a line (centering the line preceding the `\n`), there
    /// are also the escape sequences `\l` which left-justifies the
    /// preceding line and `\r` which right-justifies it.
    EscStr(Cow<'a, str>),

    /// This uses a graphviz [HTML string label][html]. The string is
    /// printed exactly as given, but between `<` and `>`. **No
    /// escaping is performed.**
    ///
    /// [html]: http://www.graphviz.org/content/node-shapes#html
    HtmlStr(Cow<'a, str>),
}

/// Escape tags in such a way that it is suitable for inclusion in a
/// Graphviz HTML label.
pub fn escape_html(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

impl<'a> LabelText<'a> {
    fn escape_char<F>(c: char, mut f: F)
        where F: FnMut(char)
    {
        match c {
            // not escaping \\, since Graphviz escString needs to
            // interpret backslashes; see EscStr above.
            '\\' => f(c),
            _ => for c in c.escape_default() {
                f(c)
            },
        }
    }
    fn escape_str(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            LabelText::escape_char(c, |c| out.push(c));
        }
        out
    }

    fn escape_default(s: &str) -> String {
        s.chars().flat_map(|c| c.escape_default()).collect()
    }

    /// Renders text as string suitable for a label in a .dot file.
    /// This includes quotes or suitable delimeters.
    pub fn to_dot_string(&self) -> String {
        match self {
            &LabelStr(ref s) => format!("\"{}\"", LabelText::escape_default(s)),
            &EscStr(ref s) => format!("\"{}\"", LabelText::escape_str(&s[..])),
            &HtmlStr(ref s) => format!("<{}>", s),
        }
    }
}