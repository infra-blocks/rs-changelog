mod state;

use state::{
    IdleNextState, IdleState, WithLabelAndDestinationParsingTitleState,
    WithLabelAndDestinationState, WithLabelNextState, WithLabelState,
};

use crate::{
    internal::parse::parser::{Finalize, Ingest, IngestResult},
    Segment,
};

use super::LinkReferenceDefinition;

pub enum LinkReferenceDefinitionParserFailure<'a> {
    Complete(Vec<Segment<'a>>),
    Partial(LinkReferenceDefinition<'a>, Segment<'a>),
}

impl<'a> LinkReferenceDefinitionParserFailure<'a> {
    fn complete(segments: Vec<Segment<'a>>) -> Self {
        Self::Complete(segments)
    }

    fn partial(
        link_reference_definition: LinkReferenceDefinition<'a>,
        segment: Segment<'a>,
    ) -> Self {
        Self::Partial(link_reference_definition, segment)
    }
}

impl<'a> From<Vec<Segment<'a>>> for LinkReferenceDefinitionParserFailure<'a> {
    fn from(segments: Vec<Segment<'a>>) -> Self {
        Self::complete(segments)
    }
}

impl<'a> From<(LinkReferenceDefinition<'a>, Segment<'a>)>
    for LinkReferenceDefinitionParserFailure<'a>
{
    fn from(
        (link_reference_definition, segment): (LinkReferenceDefinition<'a>, Segment<'a>),
    ) -> Self {
        Self::partial(link_reference_definition, segment)
    }
}

pub enum LinkReferenceDefinitionParser<'a> {
    Idle(IdleState<'a>),
    WithLabel(WithLabelState<'a>),
    WithLabelAndDestination(WithLabelAndDestinationState<'a>),
    WithLabelAndDestinationParsingTitle(WithLabelAndDestinationParsingTitleState<'a>),
}

impl<'a> LinkReferenceDefinitionParser<'a> {
    pub fn new() -> Self {
        Self::Idle(IdleState::new())
    }
}

impl<'a> From<WithLabelNextState<'a>> for LinkReferenceDefinitionParser<'a> {
    fn from(value: WithLabelNextState<'a>) -> Self {
        match value {
            WithLabelNextState::WithLabelAndDestination(state) => {
                Self::WithLabelAndDestination(state)
            }
            WithLabelNextState::WithLabelAndDestinationParsingTitle(state) => {
                Self::WithLabelAndDestinationParsingTitle(state)
            }
        }
    }
}

impl<'a> From<IdleNextState<'a>> for LinkReferenceDefinitionParser<'a> {
    fn from(value: IdleNextState<'a>) -> Self {
        match value {
            IdleNextState::WithLabel(state) => Self::WithLabel(state),
            IdleNextState::WithLabelAndDestination(state) => Self::WithLabelAndDestination(state),
            IdleNextState::WithLabelAndDestinationParsingTitle(state) => {
                Self::WithLabelAndDestinationParsingTitle(state)
            }
        }
    }
}

impl<'a> From<WithLabelAndDestinationParsingTitleState<'a>> for LinkReferenceDefinitionParser<'a> {
    fn from(value: WithLabelAndDestinationParsingTitleState<'a>) -> Self {
        LinkReferenceDefinitionParser::WithLabelAndDestinationParsingTitle(value)
    }
}

impl<'a> Ingest for LinkReferenceDefinitionParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = LinkReferenceDefinition<'a>;
    type Failure = LinkReferenceDefinitionParserFailure<'a>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            LinkReferenceDefinitionParser::Idle(state) => state
                .ingest(segment)
                .map_ready(Self::from)
                .map_failure(LinkReferenceDefinitionParserFailure::from),
            LinkReferenceDefinitionParser::WithLabel(state) => state
                .ingest(segment)
                .map_ready(Self::from)
                .map_failure(LinkReferenceDefinitionParserFailure::from),
            // Unlike other states, when there is a failure here, we return a partial failure with a single segment.
            // Therefore, we will only mutate the accumulated segments so far in the cases of success.
            LinkReferenceDefinitionParser::WithLabelAndDestination(state) => state
                .ingest(segment)
                .map_ready(Self::from)
                .map_failure(LinkReferenceDefinitionParserFailure::from),
            // We keep consuming segments until the title parser finalizes.
            // If it succeeds, everything so far has been good. If it fails, the whole process fails
            // with the entirety of the segments accumulated so far.
            LinkReferenceDefinitionParser::WithLabelAndDestinationParsingTitle(state) => state
                .ingest(segment)
                .map_ready(Self::from)
                .map_failure(LinkReferenceDefinitionParserFailure::from),
        }
    }
}

pub enum LinkReferenceDefinitionParserFinalizeResult<'a> {
    Failure(Vec<Segment<'a>>),
    // This only happens when we have the link label and destination, and we are not in the process of building a title.
    Success(LinkReferenceDefinition<'a>),
}

impl<'a> Finalize for LinkReferenceDefinitionParser<'a> {
    type Result = LinkReferenceDefinitionParserFinalizeResult<'a>;

    fn finalize(self) -> Self::Result {
        match self {
            LinkReferenceDefinitionParser::Idle(_) => {
                LinkReferenceDefinitionParserFinalizeResult::Failure(vec![])
            }
            LinkReferenceDefinitionParser::WithLabel(state) => {
                LinkReferenceDefinitionParserFinalizeResult::Failure(state.finalize())
            }
            LinkReferenceDefinitionParser::WithLabelAndDestination(state) => {
                LinkReferenceDefinitionParserFinalizeResult::Success(state.finalize())
            }
            LinkReferenceDefinitionParser::WithLabelAndDestinationParsingTitle(state) => {
                LinkReferenceDefinitionParserFinalizeResult::Failure(state.finalize())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: all dem tests.
}

/*
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
                use crate::maybe_from::MaybeInto;

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
                use crate::maybe_from::MaybeInto;

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
                use crate::maybe_from::MaybeInto;

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
 */
