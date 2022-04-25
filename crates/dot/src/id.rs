use std::borrow::Cow;
use crate::{
    node::{Node}
};

/// `Id` is a Graphviz `ID`.
pub struct Id<'a> {
    name: Cow<'a, str>,
}

impl<'a> Id<'a> {
    /// Creates an `Id` named `name`.
    ///
    /// The caller must ensure that the input conforms to an
    /// identifier format: it must be a non-empty string made up of
    /// alphanumeric or underscore characters, not beginning with a
    /// digit (i.e. the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`).
    ///
    /// (Note: this format is a strict subset of the `ID` format
    /// defined by the DOT language.  This function may change in the
    /// future to accept a broader subset, or the entirety, of DOT's
    /// `ID` format.)
    ///
    /// Passing an invalid string (containing spaces, brackets,
    /// quotes, ...) will return an empty `Err` value.
    pub fn new<Name: Into<Cow<'a, str>>>(name: Name) -> Result<Id<'a>, ()> {
        let name = name.into();
        {
            let mut chars = name.chars();
            match chars.next() {
                Some(c) if is_letter_or_underscore(c) => {}
                _ => return Err(()),
            }
            if !chars.all(is_constituent) {
                return Err(())
            }
        }
        return Ok(Id{ name: name });

        fn is_letter_or_underscore(c: char) -> bool {
            in_range('a', c, 'z') || in_range('A', c, 'Z') || c == '_'
        }
        fn is_constituent(c: char) -> bool {
            is_letter_or_underscore(c) || in_range('0', c, '9')
        }
        fn in_range(low: char, c: char, high: char) -> bool {
            low as usize <= c as usize && c as usize <= high as usize
        }
    }

    pub fn as_slice(&'a self) -> &'a str {
        &*self.name
    }

    pub fn name(self) -> Cow<'a, str> {
        self.name
    }
}

pub fn id_name<'a>(n: &Node) -> Id<'a> {
    Id::new(format!("N{}", *n)).unwrap()
}