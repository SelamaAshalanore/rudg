use std::io::prelude::*;
use std::io;

pub fn quote_string(s: String) -> String {
    format!("\"{}\"", s)
}

pub fn indent<W: Write>(w: &mut W) -> io::Result<()> {
    w.write_all(b"    ")
}