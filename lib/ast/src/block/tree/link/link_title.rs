use std::sync::LazyLock;

use crate::{
    block::tree::{
        blank_line::BlankLineNode,
        parser::{ParseResult, Parser, ParserState},
    },
    Segment,
};

/// This struct represents a link title as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-title).
///
/// It can be constructed using a [LinkTitleParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTitle<'a> {
    /// The kind of link title.
    kind: LinkTitleKind,
    /// The segments forming the link title.
    segments: Vec<Segment<'a>>,
}

impl<'a> LinkTitle<'a> {
    fn new(kind: LinkTitleKind, segments: Vec<Segment<'a>>) -> Self {
        Self { kind, segments }
    }

    // TODO: make sure this isn't available in the public API.
    /// Returns an embeddable regex string that matches a link label. No capture groups are used,
    /// but the caller shouldn't assume that. Always use named capture groups when embedding regexes.
    /// In addition, matching this regex does *not* guarantee that the text is valid. This regex
    /// only identifies where a [LinkTitle] could potentially start. It could also end on the same,
    /// segment, or not. The [LinkTitleParser] is the ultimate entity responsible for correcly.
    /// parsing a link title. Once the regex has identified a match in a relevant structure,
    /// delegate the rest of the parsing to the [LinkTitleParser].
    pub(crate) fn inline_entry_regex_str() -> &'static str {
        INLINE_ENTRY_REGEX.as_str()
    }

    pub(crate) fn inline_continuation_regex_str() -> &'static str {
        INLINE_CONTINUATION_REGEX.as_str()
    }
}

impl<'a> TryFrom<Vec<Segment<'a>>> for LinkTitle<'a> {
    type Error = Vec<Segment<'a>>;

    fn try_from(segments: Vec<Segment<'a>>) -> Result<Self, Self::Error> {
        let mut iter = segments.iter().copied();
        let Some(first_segment) = iter.next() else {
            return Err(segments);
        };
        match LinkTitleParser::start_with(first_segment) {
            ParserState::Finalized(ParseResult::Parsed(title)) => Ok(title),
            ParserState::Finalized(ParseResult::Rejected(segments)) => Err(segments),
            ParserState::Ready(parser) => match parser.consume_all(&mut iter) {
                ParseResult::Parsed(link_title) => Ok(link_title),
                ParseResult::Rejected(segments) => Err(segments),
            },
        }
    }
}

static INLINE_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            "(?:(?:{})|(?:{})|(?:{}))",
            INLINE_DOUBLE_QUOTE_ENTRY_REGEX.as_str(),
            INLINE_SINGLE_QUOTE_ENTRY_REGEX.as_str(),
            INLINE_PARENTHESIS_ENTRY_REGEX.as_str(),
        )
        .as_str(),
    )
    .unwrap()
});
static INLINE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            "(?:(?:{})|(?:{})|(?:{}))",
            INLINE_DOUBLE_QUOTE_CONTINUATION_REGEX.as_str(),
            INLINE_SINGLE_QUOTE_CONTINUATION_REGEX.as_str(),
            INLINE_PARENTHESIS_CONTINUATION_REGEX.as_str(),
        )
        .as_str(),
    )
    .unwrap()
});

// The regexes underneath are split into 6:
// - 3 regexes to match the first segment of a link title, where the opening symbol is mandatory.
// - 3 regexes to match the continuation segments of a link title, where the opening symbol is absent
// unless escaped,
//
// Each one of double quotes, single quotes and parentheses have their own pair of regexes.
// Each regex function the same: require the presence of the opening symbol on the first segment,
// tolerate escape sequences included the escaped symbol, and eventually match the closing symbol
// at the end of the segment.
//
// Note that blank lines are always invalid segments for link titles.
static INLINE_DOUBLE_QUOTE_ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#""(?:(?:\\.)|(?:[^\\"]))*(")?"#).unwrap());
static INLINE_DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"(?:(?:\\.)|(?:[^\\"]))*(")?"#).unwrap());
static DOUBLE_QUOTE_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_DOUBLE_QUOTE_ENTRY_REGEX.as_str()).as_str()).unwrap()
});
static DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_DOUBLE_QUOTE_CONTINUATION_REGEX.as_str()).as_str())
        .unwrap()
});

static INLINE_SINGLE_QUOTE_ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"'(?:(?:\\.)|(?:[^\\']))*(')?").unwrap());
static INLINE_SINGLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?:(?:\\.)|(?:[^\\']))*(')?").unwrap());
static SINGLE_QUOTE_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_SINGLE_QUOTE_ENTRY_REGEX.as_str()).as_str()).unwrap()
});
static SINGLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}?$", INLINE_SINGLE_QUOTE_CONTINUATION_REGEX.as_str()).as_str())
        .unwrap()
});

static INLINE_PARENTHESIS_ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\((?:(?:\\.)|(?:[^\\()]))*(\))?").unwrap());
static INLINE_PARENTHESIS_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?:(?:\\.)|(?:[^\\()]))*(\))?").unwrap());
static PARENTHESIS_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_PARENTHESIS_ENTRY_REGEX.as_str()).as_str()).unwrap()
});
static PARENTHESIS_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_PARENTHESIS_CONTINUATION_REGEX.as_str()).as_str())
        .unwrap()
});

/// This enum identifies the possible kinds of link titles.
///
/// It is obtained by parsing the first segment of a link title using a [LinkTitleParser].
/// Once an instance is created, it is used to validate subsequent segments of
/// the same kind until a closing symbol is found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleKind {
    DoubleQuoted,
    SingleQuoted,
    Parenthesized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValidationResult {
    Invalid,
    /// The segment is valid and the bool indicates whether it's a closing segment.
    Valid(bool),
}

impl LinkTitleKind {
    fn from_first_segment(segment: Segment) -> Option<(Self, bool)> {
        if let Some(captures) = DOUBLE_QUOTE_ENTRY_REGEX.captures(segment.text()) {
            let is_closing = captures.get(1).is_some();
            return Some((Self::DoubleQuoted, is_closing));
        }
        if let Some(captures) = SINGLE_QUOTE_ENTRY_REGEX.captures(segment.text()) {
            let is_closing = captures.get(1).is_some();
            return Some((Self::SingleQuoted, is_closing));
        }
        if let Some(captures) = PARENTHESIS_ENTRY_REGEX.captures(segment.text()) {
            let is_closing = captures.get(1).is_some();
            return Some((Self::Parenthesized, is_closing));
        }
        None
    }

    fn validate_segment(&self, segment: Segment) -> ValidationResult {
        // TODO: this should really be a utility function outside the node.
        if BlankLineNode::try_from(segment).is_ok() {
            return ValidationResult::Invalid;
        }

        let regex = match self {
            Self::DoubleQuoted => &DOUBLE_QUOTE_CONTINUATION_REGEX,
            Self::SingleQuoted => &SINGLE_QUOTE_CONTINUATION_REGEX,
            Self::Parenthesized => &PARENTHESIS_CONTINUATION_REGEX,
        };

        match regex.captures(segment.text()) {
            Some(captures) => {
                let is_closing = captures.get(1).is_some();
                ValidationResult::Valid(is_closing)
            }
            None => ValidationResult::Invalid,
        }
    }
}

/// A [Parser] for [LinkTitle].
///
/// It can be obtained using the [LinkTitleParser::start_with] method, which initializes
/// the parser with the first segment of the link title.
///
/// A link title can potentially span several lines, and a failure to parse could occur
/// at one any segment. For this reason, this parser returns a [ParseResult] as
/// its [ParserState::Finalized] variant.
///
/// When a failure occurs, the parser returns all the segments consumed so far as an [Err],
/// including the one that caused the failure. This event could happen on the first segment
/// just as much as it could occur on the 25th segment.
///
/// A successful parse returns an [Ok] variant containing the parsed [LinkTitle].
///
/// Blank lines are always invalid link title segments. Furthermore, it is expected that
/// the first character of the first segment is the opening symbol of the link title,
/// and that the last character of the last segment is the closing symbol of the link title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTitleParser<'a> {
    segments: Vec<Segment<'a>>,
    kind: LinkTitleKind,
}

impl<'a> LinkTitleParser<'a> {
    fn new(kind: LinkTitleKind, segments: Vec<Segment<'a>>) -> Self {
        Self { kind, segments }
    }

    /// Initializes the parser with the first segment of the link title.
    ///
    /// If the segment is not a valid first segment for a link title, then the
    /// parser is [ParserState::Finalized] with an [Err] containing the provided segment.
    ///
    /// If the segment is a valid first segment and also contains the closing symbol,
    /// in other words, if the link title is contained entirely in the first segment,
    /// then the parser is [ParserState::Finalized] with an [Ok] containing the parsed [LinkTitle].
    ///
    /// Finally, if the segment is a valid first segment but is expected to span multiple
    /// lines, then this function returns a [ParserState::Ready] with the initialized parser.
    pub fn start_with(segment: Segment<'a>) -> ParserState<Self, ParseResult<'a, LinkTitle<'a>>> {
        let mut segments = Vec::new();
        segments.push(segment);

        match LinkTitleKind::from_first_segment(segment) {
            Some((kind, is_closing)) => {
                if is_closing {
                    return ParserState::Finalized(ParseResult::Parsed(LinkTitle::new(
                        kind, segments,
                    )));
                }
                ParserState::Ready(Self::new(kind, segments))
            }
            None => ParserState::Finalized(ParseResult::Rejected(segments)),
        }
    }
}

impl<'a> Parser<'a> for LinkTitleParser<'a> {
    // In case of success, the parser returns a LinkTitle instance.
    // In case of failure, the parser returns all the segments that
    // were fed to it.
    type Result = ParseResult<'a, LinkTitle<'a>>;

    fn consume(
        self,
        segment: Segment<'a>,
    ) -> crate::block::tree::parser::ParserState<Self, Self::Result> {
        let mut segments = self.segments;
        segments.push(segment);

        match self.kind.validate_segment(segment) {
            ValidationResult::Invalid => ParserState::Finalized(ParseResult::Rejected(segments)),
            ValidationResult::Valid(is_closing) => {
                if is_closing {
                    ParserState::Finalized(ParseResult::Parsed(LinkTitle::new(self.kind, segments)))
                } else {
                    ParserState::Ready(Self::new(self.kind, segments))
                }
            }
        }
    }

    // If the parser is stopped before it has reached the closing matching symbol,
    // then it's always going to be incomplete and return the consumed segments.
    fn finalize(self) -> Self::Result {
        ParseResult::Rejected(self.segments)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parser {
        use super::*;

        mod start_with {
            use super::*;

            macro_rules! expect_error {
                ($name: ident, $text: expr) => {
                    #[test]
                    fn $name() {
                        let segment = Segment::first($text);
                        let state = LinkTitleParser::start_with(segment);
                        let result = state.unwrap_finalized();
                        assert_eq!(result.unwrap_rejected(), vec![segment]);
                    }
                };
            }

            macro_rules! expect_ready {
                ($name: ident, $text: expr, $kind: expr) => {
                    #[test]
                    fn $name() {
                        let segment = Segment::first($text);
                        let state = LinkTitleParser::start_with(segment);
                        let parser = state.unwrap_ready();
                        assert_eq!(parser.kind, $kind);
                        assert_eq!(parser.segments, vec![segment]);
                    }
                };
            }

            macro_rules! expect_success {
                ($name: ident, $text: expr, $kind: expr) => {
                    #[test]
                    fn $name() {
                        let segment = Segment::first($text);
                        let state = LinkTitleParser::start_with(segment);
                        let parsed = state.unwrap_finalized().unwrap_parsed();
                        assert_eq!(parsed, LinkTitle::new($kind, vec![segment]));
                    }
                };
            }

            expect_error!(should_reject_empty_line, "");
            expect_error!(should_reject_blank_line, "\n");

            mod double_quoted {
                use super::*;

                mod error {
                    use super::*;

                    expect_error!(should_reject_leading_whitespace, " \"\n");

                    expect_error!(should_reject_trailing_whitespace, "\"\" ");
                }

                mod ready {
                    use super::*;

                    expect_ready!(
                        should_work_with_single_opening_quote,
                        "\"\n",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_ready!(
                        should_work_with_some_text,
                        "\"Hello,\n",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_ready!(
                        should_work_with_escaped_quotes,
                        "\"Hello, \\\"Bro\\\"\n",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_ready!(
                        should_work_with_any_escape,
                        "\"Hello, \\;World!\n",
                        LinkTitleKind::DoubleQuoted
                    );
                }

                mod success {
                    use super::*;

                    expect_success!(
                        should_work_with_empty_quotes,
                        "\"\"",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_success!(
                        should_work_with_valid_regular_title,
                        "\"Hello, World!\"",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_success!(
                        should_work_with_escaped_quotes,
                        "\"Hello, \\\"World!\\\"\"",
                        LinkTitleKind::DoubleQuoted
                    );

                    expect_success!(
                        should_work_with_any_escape,
                        "\"Hello, \\;World!\"",
                        LinkTitleKind::DoubleQuoted
                    );
                }
            }

            mod single_quoted {
                use super::*;

                mod error {
                    use super::*;

                    expect_error!(should_reject_leading_whitespace, " '\n");

                    expect_error!(should_reject_trailing_whitespace, "'' ");
                }

                mod ready {
                    use super::*;

                    expect_ready!(
                        should_work_with_single_opening_quote,
                        "'\n",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_ready!(
                        should_work_with_some_text,
                        "'Hello,\n",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_ready!(
                        should_work_with_escaped_quotes,
                        "'Hello, \\'Bro\\'\n",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_ready!(
                        should_work_with_any_escape,
                        "'Hello, \\;World!\n",
                        LinkTitleKind::SingleQuoted
                    );
                }

                mod success {
                    use super::*;

                    expect_success!(
                        should_work_with_empty_quotes,
                        "''",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_success!(
                        should_work_with_valid_regular_title,
                        "'Hello, World!'",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_success!(
                        should_work_with_escaped_quotes,
                        "'Hello, \\'World!\\''",
                        LinkTitleKind::SingleQuoted
                    );

                    expect_success!(
                        should_work_with_any_escape,
                        "'Hello, \\;World!'",
                        LinkTitleKind::SingleQuoted
                    );
                }
            }

            mod parenthesized {
                use super::*;

                mod error {
                    use super::*;

                    expect_error!(should_reject_leading_whitespace, " (\n");

                    expect_error!(should_reject_trailing_whitespace, "() ");
                }

                mod ready {
                    use super::*;

                    expect_ready!(
                        should_work_with_single_opening_parenthesis,
                        "(\n",
                        LinkTitleKind::Parenthesized
                    );

                    expect_ready!(
                        should_work_with_some_text,
                        "(Hello,\n",
                        LinkTitleKind::Parenthesized
                    );

                    expect_ready!(
                        should_work_with_escaped_parentheses,
                        "(Hello, \\(Bro\\)\n",
                        LinkTitleKind::Parenthesized
                    );

                    expect_ready!(
                        should_work_with_any_escape,
                        "(Hello, \\;World!\n",
                        LinkTitleKind::Parenthesized
                    );
                }

                mod success {
                    use super::*;

                    expect_success!(
                        should_work_with_empty_parentheses,
                        "()",
                        LinkTitleKind::Parenthesized
                    );

                    expect_success!(
                        should_work_with_valid_regular_title,
                        "(Hello, World!)",
                        LinkTitleKind::Parenthesized
                    );

                    expect_success!(
                        should_work_with_escaped_parentheses,
                        "(Hello, \\(World!\\))",
                        LinkTitleKind::Parenthesized
                    );

                    expect_success!(
                        should_work_with_any_escape,
                        "(Hello, \\;World!)",
                        LinkTitleKind::Parenthesized
                    );
                }
            }
        }

        mod consume {
            use super::*;

            use crate::segment::StrExt;

            // Expect error on the last line.
            macro_rules! expect_error {
                ($name: ident, $text: expr) => {
                    #[test]
                    fn $name() {
                        let segments: Vec<_> = $text.line_segments().collect();
                        let mut parser = LinkTitleParser::start_with(segments[0]).unwrap_ready();
                        for i in 1..segments.len() - 1 {
                            parser = parser.consume(segments[i]).unwrap_ready();
                        }
                        let result = parser
                            .consume(segments[segments.len() - 1])
                            .unwrap_finalized();
                        assert_eq!(result.unwrap_rejected(), segments);
                    }
                };
            }

            // TODO: implement
            macro_rules! expect_ready {
                ($name: ident, $text: expr, $kind: expr) => {
                    #[test]
                    fn $name() {
                        let segment = Segment::first($text);
                        let state = LinkTitleParser::start_with(segment);
                        let parser = state.unwrap_ready();
                        assert_eq!(parser.kind, $kind);
                        assert_eq!(parser.segments, vec![segment]);
                    }
                };
            }

            // TODO: implement
            macro_rules! expect_success {
                ($name: ident, $text: expr, $kind: expr) => {
                    #[test]
                    fn $name() {
                        let segment = Segment::first($text);
                        let state = LinkTitleParser::start_with(segment);
                        let parsed = state.unwrap_finalized().unwrap();
                        assert_eq!(parsed, LinkTitle::new($kind, vec![segment]));
                    }
                };
            }

            mod double_quoted {
                use super::*;

                mod error {
                    use super::*;

                    expect_error!(should_reject_blank_line, "\"\n\n");
                }

                #[test]
                fn should_work_with_single_newline() {
                    let segments: Vec<_> = r#""
""#
                    .line_segments()
                    .collect();

                    let parser = LinkTitleParser::start_with(segments[0]).unwrap_ready();
                    let state = parser.consume(segments[1]);
                    let result = state.unwrap_finalized();
                    assert_eq!(
                        result.unwrap_parsed(),
                        LinkTitle::new(LinkTitleKind::DoubleQuoted, segments)
                    );
                }

                #[test]
                fn should_work_with_regular_text() {
                    let segments: Vec<_> = r#""Hello,
World!""#
                        .line_segments()
                        .collect();
                    let parser = LinkTitleParser::start_with(segments[0]).unwrap_ready();
                    let state = parser.consume(segments[1]);
                    let result = state.unwrap_finalized();
                    assert_eq!(
                        result.unwrap_parsed(),
                        LinkTitle::new(LinkTitleKind::DoubleQuoted, segments)
                    );
                }

                #[test]
                fn should_work_with_escaped_quotes() {
                    let segments: Vec<_> = r#""Hello,
\"World!\"""#
                        .line_segments()
                        .collect();
                    let parser = LinkTitleParser::start_with(segments[0]).unwrap_ready();
                    let state = parser.consume(segments[1]);
                    let result = state.unwrap_finalized();
                    assert_eq!(
                        result.unwrap_parsed(),
                        LinkTitle::new(LinkTitleKind::DoubleQuoted, segments)
                    );
                }

                #[test]
                fn should_work_with_any_escape() {
                    let segments: Vec<_> = r#""Hello,
\;World!""#
                        .line_segments()
                        .collect();
                    let parser = LinkTitleParser::start_with(segments[0]).unwrap_ready();
                    let state = parser.consume(segments[1]);
                    let result = state.unwrap_finalized();
                    assert_eq!(
                        result.unwrap_parsed(),
                        LinkTitle::new(LinkTitleKind::DoubleQuoted, segments)
                    );
                }
            }
        }

        // TODO: make sure all the test modules cover all the cases.
        // TODO: continuation tests.
    }
}
