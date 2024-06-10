use markdown::mdast::Node;

/// Taken from markdown's source code, it simply isn't exposed.
pub fn children_to_string(children: &[Node]) -> String {
    children.iter().map(ToString::to_string).collect()
}
