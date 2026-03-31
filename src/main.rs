use std::{borrow::Cow, path::Path};

use changelog::{debug, parse_ast, Node, Tree};
use clap::{arg, Command};
use miette::{IntoDiagnostic, Result};
use ptree::{print_tree, TreeItem};

fn main() -> Result<()> {
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
            PrettyTree::from(&tree).pretty_print().into_diagnostic()?;
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
struct PrettyTree<'a, 'b: 'a>(&'b Tree<'a>);

impl<'a, 'b: 'a> PrettyTree<'a, 'b> {
    fn pretty_print(&self) -> std::io::Result<()> {
        print_tree(self)
    }
}

impl<'a, 'b: 'a> From<&'b Tree<'a>> for PrettyTree<'a, 'b> {
    fn from(value: &'b Tree<'a>) -> Self {
        PrettyTree(value)
    }
}

impl<'a, 'b: 'a> TreeItem for PrettyTree<'a, 'b> {
    type Child = PrettyNode<'a, 'b>;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint("Document"))
    }

    fn children(&self) -> Cow<'_, [Self::Child]> {
        Cow::from(pretty_nodes(&self.0.branches))
    }
}

#[derive(Clone)]
struct PrettyNode<'a, 'b: 'a>(&'b Node<'a>);

impl<'a, 'b: 'a> TreeItem for PrettyNode<'a, 'b> {
    type Child = Self;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(
            f,
            "{}",
            style.paint(format!("{:?}", (&self.0.event, &self.0.range)))
        )
    }

    fn children(&self) -> Cow<'_, [Self::Child]> {
        Cow::from(pretty_nodes(&self.0.children))
    }
}

fn pretty_nodes<'a, 'b: 'a>(children: &'b [Node<'a>]) -> Vec<PrettyNode<'a, 'b>> {
    children.iter().map(PrettyNode).collect()
}
