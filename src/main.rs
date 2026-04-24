use std::{borrow::Cow, path::Path};

use changelog::{debug, parse_ast};
use changelog_ast::{Ast, Node};
use clap::{arg, Command};
use miette::{IntoDiagnostic, Result};
use ptree::{print_tree, TreeItem};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let command = Command::new("rs-changelog")
        .author("El Pendeloco")
        .subcommand(
            Command::new("ast")
                .about("This program takes in a markdown file name and produces an AST.")
                .arg(arg!(<file> "The markdown file to parse.")),
        )
        .subcommand(
            Command::new("debug")
                .about("This command outputs the result of running the file through pulldown-cmark")
                .arg(arg!(<file> "The markdown file to parse.")),
        )
        .after_help("This program is a work in progress.");
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("ast", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            let tree = parse_ast(&content);
            PrettyAst::from(&tree).pretty_print().into_diagnostic()?;
        }
        Some(("debug", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            debug(&content);
        }
        Some((unknown, _)) => panic!("unknown subcommand: {}", unknown),
        None => panic!("unexpected lack of subcommand"),
    };
    Ok(())
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    std::fs::read_to_string(&path).into_diagnostic()
}

#[derive(Clone)]
struct PrettyAst<'ast, 'source: 'ast>(&'ast Ast<'source>);

impl<'ast, 'source: 'ast> PrettyAst<'ast, 'source> {
    fn pretty_print(&self) -> std::io::Result<()> {
        print_tree(self)
    }
}

impl<'ast, 'source: 'ast> From<&'ast Ast<'source>> for PrettyAst<'ast, 'source> {
    fn from(value: &'ast Ast<'source>) -> Self {
        PrettyAst(value)
    }
}

impl<'ast, 'source: 'ast> TreeItem for PrettyAst<'ast, 'source> {
    type Child = PrettyNode<'ast, 'source>;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint("Ast"))
    }

    fn children(&self) -> Cow<'_, [Self::Child]> {
        Cow::from(self.0.nodes.iter().map(PrettyNode).collect::<Vec<_>>())
    }
}

// TODO: implement with block or inline.
#[derive(Clone)]
struct PrettyNode<'ast, 'source: 'ast>(&'ast Node<'source>);

impl<'ast, 'source: 'ast> TreeItem for PrettyNode<'ast, 'source> {
    type Child = Self;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint(format!("{:?}", &self.0)))
    }

    fn children(&self) -> Cow<'_, [Self::Child]> {
        Cow::from(match self.0 {
            Node::Leaf(_) => vec![],
            Node::Internal(internal) => internal.children.iter().map(PrettyNode).collect(),
        })
    }
}
