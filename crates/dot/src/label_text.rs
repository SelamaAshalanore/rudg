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
    pub fn label<S:Into<Cow<'a, str>>>(s: S) -> LabelText<'a> {
        LabelStr(s.into())
    }

    pub fn escaped<S:Into<Cow<'a, str>>>(s: S) -> LabelText<'a> {
        EscStr(s.into())
    }

    pub fn html<S: Into<Cow<'a, str>>>(s: S) -> LabelText<'a> {
        HtmlStr(s.into())
    }

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

    /// Decomposes content into string suitable for making EscStr that
    /// yields same content as self.  The result obeys the law
    /// render(`lt`) == render(`EscStr(lt.pre_escaped_content())`) for
    /// all `lt: LabelText`.
    fn pre_escaped_content(self) -> Cow<'a, str> {
        match self {
            EscStr(s) => s,
            LabelStr(s) => if s.contains('\\') {
                LabelText::escape_default(&*s).into()
            } else {
                s
            },
            HtmlStr(s) => s,
        }
    }

    /// Puts `prefix` on a line above this label, with a blank line separator.
    pub fn prefix_line(self, prefix: LabelText) -> LabelText<'static> {
        prefix.suffix_line(self)
    }

    /// Puts `suffix` on a line below this label, with a blank line separator.
    pub fn suffix_line(self, suffix: LabelText) -> LabelText<'static> {
        let mut prefix = self.pre_escaped_content().into_owned();
        let suffix = suffix.pre_escaped_content();
        prefix.push_str(r"\n\n");
        prefix.push_str(&suffix[..]);
        EscStr(prefix.into())
    }
}