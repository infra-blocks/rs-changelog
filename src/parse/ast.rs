use std::collections::VecDeque;

use changelog_ast::Node;

pub type Ast<'source> = VecDeque<Node<'source>>;
