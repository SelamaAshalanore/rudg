pub mod to_dot;

pub trait GraphExporter {
    fn to_string(&self) -> String;
}