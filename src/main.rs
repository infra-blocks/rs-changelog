mod ast;

use std::{borrow::Cow, path::Path};

use changelog::{check, debug};
use changelog_ast::Node;
use clap::{Command, arg};
use miette::{IntoDiagnostic, Result};
use ptree::{TreeItem, print_tree};

use crate::ast::{Ast, parse_ast};

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
        .subcommand(
            Command::new("check")
                .about("This command checks the provided changelog.")
                .arg(arg!(<file> "The mardkwon file to lint.")),
        )
        .after_help("This program is a work in progress.");
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("ast", args)) => {
            let file = args.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            let tree = parse_ast(&content);
            PrettyAst::from(&tree).pretty_print().into_diagnostic()?;
            for (key, value) in tree.reference_definitions.iter() {
                println!("{} => {:?}: {}", key, value, &content[value.span.clone()]);
            }
        }
        Some(("debug", args)) => {
            let file = args.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            debug(&content);
        }
        Some(("check", args)) => {
            let file = args.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            check(&content).into_diagnostic()?;
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
        // TODO: consider providing a children iterator on nodes.
        Cow::from(match self.0 {
            Node::BlockQuote(inner) => inner.children.iter().map(Self).collect(),
            Node::CodeBlock(inner) => inner.children.iter().map(Self).collect(),
            Node::DefinitionList(inner) => inner.children.iter().map(Self).collect(),
            Node::DefinitionListTitle(inner) => inner.children.iter().map(Self).collect(),
            Node::DefinitionListDefinition(inner) => inner.children.iter().map(Self).collect(),
            Node::Emphasis(inner) => inner.children.iter().map(Self).collect(),
            Node::FootnoteDefinition(inner) => inner.children.iter().map(Self).collect(),
            Node::Heading(inner) => inner.children.iter().map(Self).collect(),
            Node::HtmlBlock(inner) => inner.children.iter().map(Self).collect(),
            Node::Image(inner) => inner.children.iter().map(Self).collect(),
            Node::Item(inner) => inner.children.iter().map(Self).collect(),
            Node::Link(inner) => inner.children.iter().map(Self).collect(),
            Node::List(inner) => inner.children.iter().map(Self).collect(),
            Node::MetadataBlock(inner) => inner.children.iter().map(Self).collect(),
            Node::Paragraph(inner) => inner.children.iter().map(Self).collect(),
            Node::Strong(inner) => inner.children.iter().map(Self).collect(),
            Node::Strikethrough(inner) => inner.children.iter().map(Self).collect(),
            Node::Subscript(inner) => inner.children.iter().map(Self).collect(),
            Node::Superscript(inner) => inner.children.iter().map(Self).collect(),
            Node::Table(inner) => inner.children.iter().map(Self).collect(),
            Node::TableCell(inner) => inner.children.iter().map(Self).collect(),
            Node::TableHead(inner) => inner.children.iter().map(Self).collect(),
            Node::TableRow(inner) => inner.children.iter().map(Self).collect(),
            // Leaf nodes
            Node::Code(_)
            | Node::DisplayMath(_)
            | Node::FootnoteReference(_)
            | Node::HardBreak(_)
            | Node::Html(_)
            | Node::InlineHtml(_)
            | Node::InlineMath(_)
            | Node::Rule(_)
            | Node::SoftBreak(_)
            | Node::TaskListMarker(_)
            | Node::Text(_) => vec![],
        })
    }
}
