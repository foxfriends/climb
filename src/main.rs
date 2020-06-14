use std::path::PathBuf;
use std::io::Read;

mod format;
use format::Format;

mod tree;
use tree::Tree;

mod render;
use render::render;

/// Interactive tree-shaped-data navigator.
#[derive(Debug, structopt::StructOpt)]
struct Opts {
    /// Input format of the tree.
    #[structopt(short, long, default_value)]
    format: Format,
    /// Input file to read tree from. Will read from STDIN by default.
    input: Option<PathBuf>,
}

#[paw::main]
fn main(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let source = match opts.input {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let tree = match opts.format {
        Format::Sexp =>  Tree::from(sexp::parse(&source)?),
    };

    render(tree)
}
