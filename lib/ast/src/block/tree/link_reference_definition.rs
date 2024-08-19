use std::sync::LazyLock;

use super::{
    link::{LinkLabel, LinkTitle, LinkTitleParser},
    parser::{ParseResult, Parser, ParserState, PartialParseResult},
};
use crate::{block::tree::link::LinkDestination, Segment};

//TODO: to_html() for this struct won't produce anything.
// Only its usages in the rest of the document would.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinitionNode<'a> {
    pub segments: Vec<Segment<'a>>,
    pub label: LinkLabel<'a>,
    pub destination: LinkDestination<'a>,
    pub title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinitionNode<'a> {
    fn new(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: Option<LinkTitle<'a>>,
    ) -> Self {
        Self {
            segments,
            label,
            destination,
            title,
        }
    }

    fn without_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
    ) -> Self {
        Self::new(segments, label, destination, None)
    }

    fn with_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: LinkTitle<'a>,
    ) -> Self {
        Self::new(segments, label, destination, Some(title))
    }
}

// NOTE: The link title parser does handle the newline characters when they are included
// in the title. This means that, unlike most other block Regexes, we can't assume that
// a newline character will always be present at the end of the segment, outside of the
// title capture group, since the latter might have consumed it. However, when the
// title gets closed, there is necessarily a newline character following. This is why
// we include an optional newline character at the end of the regexes. In any case,
// the newline character is the only character allowed after the title has been closed.

/// This regex will match the first line of a link reference definition. The complexity here lies
/// in the fact that both the destination and the title can be started on the next lines.
static ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            r"^[ ]{{0,3}}(?<label>{}):\s*(?:(?<destination>{})\s*)?(?:\s+(?<title>{}))?\n?$",
            LinkLabel::inline_regex_str(),
            LinkDestination::inline_regex_str(),
            LinkTitle::inline_entry_regex_str(),
        )
        .as_str(),
    )
    .unwrap()
});

/// This regex is to be used if the first segment only included the label. In this case,
/// it will match the segment only if it contains a mandatory destination, and
/// possibly include a partial or entire title.
static DESTINATION_AND_TITLE_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            r"^\s*(?<destination>{})\s*(?:\s+(?<title>{}))?\n?$",
            LinkDestination::inline_regex_str(),
            LinkTitle::inline_entry_regex_str(),
        )
        .as_str(),
    )
    .unwrap()
});

/// This regex is to be used when both the label and the destination have been parsed,
/// but the title has not been started yet. Basically, this regex will only match
/// if the segment contains a valid title opening/entire title.
static TITLE_ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(r"^\s*(?<title>{})\n?$", LinkTitle::inline_entry_regex_str()).as_str(),
    )
    .unwrap()
});
static TITLE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            r"^(?<title>{})\n?$",
            LinkTitle::inline_continuation_regex_str()
        )
        .as_str(),
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinitionParser<'a> {
    segments: Vec<Segment<'a>>,
    label: LinkLabel<'a>,
    // Even though the destination is actually mandatory, it could be missing from the first segment.
    destination: Option<LinkDestination<'a>>,
    title_parser: Option<LinkTitleParser<'a>>,
}

impl<'a> LinkReferenceDefinitionParser<'a> {
    fn new(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: Option<LinkDestination<'a>>,
        title_parser: Option<LinkTitleParser<'a>>,
    ) -> Self {
        Self {
            segments,
            label,
            destination,
            title_parser,
        }
    }

    fn with_label_only(segments: Vec<Segment<'a>>, label: LinkLabel<'a>) -> Self {
        Self::new(segments, label, None, None)
    }

    fn with_label_and_destination(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
    ) -> LinkReferenceDefinitionParser<'a> {
        Self::new(segments, label, Some(destination), None)
    }

    fn with_all_three(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title_parser: LinkTitleParser<'a>,
    ) -> LinkReferenceDefinitionParser<'a> {
        Self::new(segments, label, Some(destination), Some(title_parser))
    }

    pub fn start_with(
        segment: Segment<'a>,
    ) -> ParserState<Self, ParseResult<'a, LinkReferenceDefinitionNode<'a>>> {
        let segments = vec![segment];
        let captures = match ENTRY_REGEX.captures(segment.text()) {
            Some(captures) => captures,
            None => return ParserState::Finalized(ParseResult::Rejected(segments)),
        };

        let label_match = match captures.name("label") {
            Some(label_match) => label_match,
            // If we can't find a match for the label on the first segment, it is definitely
            // not a valid segment for a link reference definition.
            None => return ParserState::Finalized(ParseResult::Rejected(segments)),
        };
        let label_match_segment = Segment::new(
            segment.start().walk(&segment.text()[..label_match.start()]),
            label_match.as_str(),
        );
        let label = match LinkLabel::try_from(label_match_segment) {
            Ok(label) => label,
            Err(_) => return ParserState::Finalized(ParseResult::Rejected(segments)),
        };

        let destination_match = match captures.name("destination") {
            Some(destination_match) => destination_match,
            // To keep the regex simpler, we don't encode the cases where you *need* the link
            // destination before the link title when the title is present. Instead, we make
            // that check here. If we don't have a match for the destination, but we do
            // have one for the title, that is an invalid segment. Otherwise, if we don't
            // have a match for neither, we can return a parser ready for next segment.
            None => {
                if captures.name("title").is_some() {
                    return ParserState::Finalized(ParseResult::Rejected(segments));
                }

                return ParserState::Ready(Self::with_label_only(segments, label));
            }
        };
        let destination_match_segment = Segment::new(
            segment
                .start()
                .walk(&segment.text()[..destination_match.start()]),
            destination_match.as_str(),
        );
        let destination = match LinkDestination::try_from(destination_match_segment) {
            Ok(destination) => destination,
            // If we did have a match for the destination but turns out it is invalid, then
            // it invalidates the whole segment.
            Err(_) => return ParserState::Finalized(ParseResult::Rejected(segments)),
        };

        // Finally, we check to see if there is a title. If there is no title, we will only know
        // on the next segment if the link reference definition is valid or not. If there is a match
        // then 3 things can happen:
        // 1. The match is not actually a valid title, in which case the whole segment is invalid.
        // 2. The match is a valid title start, but the parser needs more segment. In that case, we can
        //    return a parser ready for the next segment.
        // 3. The match is a valid whole title. In this case, we can finalize the and return the
        //    valid link reference definition already.
        let title_match = match captures.name("title") {
            Some(title_match) => title_match,
            None => {
                return ParserState::Ready(Self::with_label_and_destination(
                    segments,
                    label,
                    destination,
                ))
            }
        };
        // TODO: utility function for that.
        let title_match_segment = Segment::new(
            segment.start().walk(&segment.text()[..title_match.start()]),
            title_match.as_str(),
        );
        match LinkTitleParser::start_with(title_match_segment) {
            ParserState::Finalized(ParseResult::Rejected(_)) => {
                ParserState::Finalized(ParseResult::Rejected(segments))
            }
            ParserState::Finalized(ParseResult::Parsed(title)) => {
                ParserState::Finalized(ParseResult::Parsed(
                    LinkReferenceDefinitionNode::with_title(segments, label, destination, title),
                ))
            }
            ParserState::Ready(title_parser) => ParserState::Ready(Self::with_all_three(
                segments,
                label,
                destination,
                title_parser,
            )),
        }
    }
}

impl<'a> Parser<'a> for LinkReferenceDefinitionParser<'a> {
    type Result = PartialParseResult<'a, LinkReferenceDefinitionNode<'a>>;

    fn consume(self, segment: Segment<'a>) -> ParserState<Self, Self::Result> {
        // What we do here always depends on what state the parser is in.
        // There are a few cases to cover:
        // - The destination was not provided on the first line. In this case, it will
        // be None and we should start with the destination.
        // - The destination was provided on the first line, but the title was not. In
        // this case, the destination will be set but the title parser will be None.
        // - Finally, if the title parser is Some, then we only need to delegate to
        // it.
        let mut segments = self.segments;
        segments.push(segment);

        // If there is not destination parsed yet, then the next thing to parse is a destination.
        let Some(destination) = self.destination else {
            match DESTINATION_AND_TITLE_ENTRY_REGEX.captures(segment.text()) {
                Some(captures) => {
                    // The regex forces the presence of a destination.
                    let destination_match = captures.name("destination").unwrap();
                    let destination_match_segment = Segment::new(
                        segment
                            .start()
                            .walk(&segment.text()[..destination_match.start()]),
                        destination_match.as_str(),
                    );
                    let Ok(destination) = LinkDestination::try_from(destination_match_segment)
                    else {
                        return ParserState::Finalized(PartialParseResult::Rejected(segments));
                    };

                    // If there is no title, then it must be on the next line, and we are done for now.
                    let Some(title_match) = captures.name("title") else {
                        return ParserState::Ready(Self::with_label_and_destination(
                            segments,
                            self.label,
                            destination,
                        ));
                    };

                    // If there is a title, then we need to dispatch to the title parser.
                    let title_match_segment = Segment::new(
                        segment.start().walk(&segment.text()[..title_match.start()]),
                        title_match.as_str(),
                    );
                    return match LinkTitleParser::start_with(title_match_segment) {
                        ParserState::Finalized(ParseResult::Rejected(_)) => {
                            ParserState::Finalized(PartialParseResult::Rejected(segments))
                        }
                        ParserState::Finalized(ParseResult::Parsed(title)) => {
                            ParserState::Finalized(PartialParseResult::Parsed(
                                LinkReferenceDefinitionNode::with_title(
                                    segments,
                                    self.label,
                                    destination,
                                    title,
                                ),
                            ))
                        }
                        ParserState::Ready(new_parser) => ParserState::Ready(Self::with_all_three(
                            segments,
                            self.label,
                            destination,
                            new_parser,
                        )),
                    };
                }
                None => return ParserState::Finalized(PartialParseResult::Rejected(segments)),
            };
        };

        // Here, we have a destination parsed already, but no title has been started yet.
        let Some(title_parser) = self.title_parser else {
            match TITLE_ENTRY_REGEX.captures(segment.text()) {
                Some(captures) => {
                    let title_match = captures.name("title").unwrap();
                    let title_match_segment = Segment::new(
                        segment.start().walk(&segment.text()[..title_match.start()]),
                        title_match.as_str(),
                    );
                    return match LinkTitleParser::start_with(title_match_segment) {
                        // If we couldn't parse a title, but we already have a valid label and destination, we're done.
                        // Note that this means that we *haven't* consumed the segment!
                        ParserState::Finalized(ParseResult::Rejected(_)) => {
                            // Remove the last element from the segments.
                            segments.pop();
                            ParserState::Finalized(PartialParseResult::Partial(
                                LinkReferenceDefinitionNode::without_title(
                                    segments,
                                    self.label,
                                    destination,
                                ),
                                vec![segment],
                            ))
                        }
                        ParserState::Finalized(ParseResult::Parsed(title)) => {
                            ParserState::Finalized(PartialParseResult::Parsed(
                                LinkReferenceDefinitionNode::with_title(
                                    segments,
                                    self.label,
                                    destination,
                                    title,
                                ),
                            ))
                        }
                        ParserState::Ready(new_parser) => ParserState::Ready(Self::with_all_three(
                            segments,
                            self.label,
                            destination,
                            new_parser,
                        )),
                    };
                }
                // Same scenario as when the title is invalid.
                None => {
                    segments.pop();
                    return ParserState::Finalized(PartialParseResult::Partial(
                        LinkReferenceDefinitionNode::without_title(
                            segments,
                            self.label,
                            destination,
                        ),
                        vec![segment],
                    ));
                }
            };
        };

        // If the title is already ongoing and it happens to spread over multiple lines,
        // then we use the continuation regex to match the possible extra newline character after
        // the closing symbol.
        match TITLE_CONTINUATION_REGEX.captures(segment.text()) {
            Some(captures) => {
                let title_match = captures.name("title").unwrap();
                let title_match_segment = Segment::new(
                    segment.start().walk(&segment.text()[..title_match.start()]),
                    title_match.as_str(),
                );
                match title_parser.consume(title_match_segment) {
                    // Unlike the case where we have a valid label and destination but can't *start* a valid title,
                    // if the title is ongoing and invalidated midway, it invalidates the whole thing. No link reference
                    // definition will come out.
                    ParserState::Finalized(ParseResult::Rejected(_)) => {
                        ParserState::Finalized(PartialParseResult::Rejected(segments))
                    }
                    ParserState::Finalized(ParseResult::Parsed(title)) => ParserState::Finalized(
                        PartialParseResult::Parsed(LinkReferenceDefinitionNode::with_title(
                            segments,
                            self.label,
                            destination,
                            title,
                        )),
                    ),
                    ParserState::Ready(new_parser) => ParserState::Ready(Self::with_all_three(
                        segments,
                        self.label,
                        destination,
                        new_parser,
                    )),
                }
            }
            None => ParserState::Finalized(PartialParseResult::Rejected(segments)),
        }
    }

    // TODO: test explicitly finalization.
    fn finalize(self) -> Self::Result {
        // There are 2 outcomes possible upon finalization:
        // 1. The parser is in a state that cannot complete conversion to a link reference definition.
        // 2. The parser is in a state that can complete conversion to a link reference definition.
        // The only possible for #2 to occur is that the parser already has a valid label and destination,
        // and that the title hasn't begin yet. Every other case results in a rejection.
        if self.destination.is_some() && self.title_parser.is_none() {
            PartialParseResult::Partial(
                LinkReferenceDefinitionNode::without_title(
                    self.segments,
                    self.label,
                    self.destination.unwrap(),
                ),
                vec![],
            )
        } else {
            PartialParseResult::Rejected(self.segments)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parser {
        use super::*;

        mod start_with {
            use super::*;

            #[test]
            fn should_reject_empty_segment() {
                let segment = Segment::default();
                assert_eq!(
                    LinkReferenceDefinitionParser::start_with(segment),
                    ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                );
            }

            #[test]
            fn should_reject_blank_line() {
                let segment = Segment::first("\n");
                assert_eq!(
                    LinkReferenceDefinitionParser::start_with(segment),
                    ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                );
            }

            // Note: The goal of these tests is not to repeat the battery of tests that already exist for
            // LinkLabels, LinkDestinations and LinkTitles, rather to ensure that this parser correctly
            // and dispatch the logic to each component.
            mod label_only {
                use super::*;

                #[test]
                fn should_reject_missing_colon() {
                    // The label is valid here, but the colon is missing.
                    let segment = Segment::first("[a]\n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_reject_4_spaces_indent() {
                    // One whitespace too many makes this a failure.
                    let segment = Segment::first("    [a]:\n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_reject_tab_indent() {
                    let segment = Segment::first("\t[a]:\n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_work_with_the_simplest_link_label() {
                    let segment = Segment::first("[a]:\n");
                    let parser = LinkReferenceDefinitionParser::start_with(segment).unwrap_ready();
                    assert_eq!(parser.segments, vec![segment]);
                    assert_eq!(parser.label, Segment::first("[a]").try_into().unwrap());
                }

                #[test]
                fn should_work_with_3_spaces_indent() {
                    let segment = Segment::first("   [a]:\n");
                    let parser = LinkReferenceDefinitionParser::start_with(segment).unwrap_ready();
                    assert_eq!(parser.segments, vec![segment]);
                    assert_eq!(
                        parser.label,
                        Segment::new(location::Position::new(1, 4, 3), "[a]")
                            .try_into()
                            .unwrap()
                    );
                }
            }

            mod label_and_destination {
                use super::*;

                #[test]
                fn should_reject_invalid_destination_but_valid_title() {
                    let segment = Segment::first("[a]: \"whitespaces ruin everything\"\n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_work_with_slash_destination() {
                    let segment = Segment::first("[a]: /path/to/file\n");
                    let parser = LinkReferenceDefinitionParser::start_with(segment).unwrap_ready();
                    assert_eq!(parser.segments, vec![segment]);
                    assert_eq!(parser.label, Segment::first("[a]").try_into().unwrap());
                    assert_eq!(
                        parser.destination.unwrap(),
                        Segment::new(location::Position::new(1, 6, 5), "/path/to/file")
                            .try_into()
                            .unwrap()
                    );
                }

                #[test]
                fn should_work_without_spaces_in_between_components() {
                    let segment = Segment::first("[label]:/destination\n");
                    let parser = LinkReferenceDefinitionParser::start_with(segment).unwrap_ready();
                    assert_eq!(parser.segments, vec![segment]);
                    assert_eq!(parser.label, Segment::first("[label]").try_into().unwrap());
                    assert_eq!(
                        parser.destination.unwrap(),
                        Segment::new(location::Position::new(1, 9, 8), "/destination")
                            .try_into()
                            .unwrap()
                    );
                }

                #[test]
                fn should_work_with_plenty_of_whitespaces_between_components() {
                    let segment = Segment::first("[label]:  \t /destination\n");
                    let parser = LinkReferenceDefinitionParser::start_with(segment).unwrap_ready();
                    assert_eq!(parser.segments, vec![segment]);
                    assert_eq!(parser.label, Segment::first("[label]").try_into().unwrap());
                    assert_eq!(
                        parser.destination.unwrap(),
                        Segment::new(location::Position::new(1, 13, 12), "/destination")
                            .try_into()
                            .unwrap()
                    );
                }
            }

            mod all_on_first_segment {
                use super::*;

                #[test]
                fn should_reject_trailing_whitespaces() {
                    let segment = Segment::first("[foo]: <bar> (baz)  \n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_reject_missing_whitespace_before_title() {
                    let segment = Segment::first("[foo]:<bar>(baz)\n");
                    assert_eq!(
                        LinkReferenceDefinitionParser::start_with(segment),
                        ParserState::Finalized(ParseResult::Rejected(vec![segment]))
                    );
                }

                #[test]
                fn should_work_with_whitespace_before_title() {
                    let segment = Segment::first("[foo]:<bar> (baz)\n");
                    let definition = LinkReferenceDefinitionParser::start_with(segment)
                        .unwrap_finalized()
                        .unwrap_parsed();
                    assert_eq!(definition.segments, vec![segment]);
                    assert_eq!(
                        definition.label,
                        Segment::first("[foo]").try_into().unwrap()
                    );
                    assert_eq!(
                        definition.destination,
                        Segment::new(location::Position::new(1, 7, 6), "<bar>")
                            .try_into()
                            .unwrap()
                    );
                    assert_eq!(
                        definition.title.unwrap(),
                        vec![Segment::new(location::Position::new(1, 13, 12), "(baz)")]
                            .try_into()
                            .unwrap()
                    );
                }

                #[test]
                fn should_work_with_one_whitespace_between_components() {
                    let segment = Segment::first("[foo]: <bar> (baz)\n");
                    let definition = LinkReferenceDefinitionParser::start_with(segment)
                        .unwrap_finalized()
                        .unwrap_parsed();
                    assert_eq!(definition.segments, vec![segment]);
                    assert_eq!(
                        definition.label,
                        Segment::first("[foo]").try_into().unwrap()
                    );
                    assert_eq!(
                        definition.destination,
                        Segment::new(location::Position::new(1, 8, 7), "<bar>")
                            .try_into()
                            .unwrap()
                    );
                    assert_eq!(
                        definition.title.unwrap(),
                        vec![Segment::new(location::Position::new(1, 14, 13), "(baz)")]
                            .try_into()
                            .unwrap()
                    );
                }
            }
        }
    }
}
