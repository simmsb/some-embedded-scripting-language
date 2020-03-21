use pretty::{DocAllocator, DocBuilder};
use termcolor::{Color, ColorSpec};

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(u64),   // TODO: bigints
    Float(f64), // TODO: bigdecimals
    Void,
}

impl Literal {
    pub fn pretty<'a, D>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, ColorSpec>
    where
        D: DocAllocator<'a, ColorSpec>,
        D::Doc: Clone,
    {
        match self {
            Literal::String(s) => allocator
                .text(format!("\"{}\"", s))
                .annotate(ColorSpec::new().set_fg(Some(Color::Yellow)).clone()),
            Literal::Int(v) => allocator
                .as_string(v)
                .annotate(ColorSpec::new().set_fg(Some(Color::Yellow)).clone()),
            Literal::Float(v) => allocator
                .as_string(v)
                .annotate(ColorSpec::new().set_fg(Some(Color::Yellow)).clone()),
            Literal::Void => allocator
                .text("void")
                .annotate(ColorSpec::new().set_fg(Some(Color::Yellow)).clone()),
        }
    }
}
