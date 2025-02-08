use either::Either;
use segment::{LineSegment, Segment, SegmentLike};

use crate::internal::parse::{
    link::{LinkDestination, LinkLabel, LinkTitle, LinkTitleParser},
    parser::{Ingest, IngestResult},
    try_extract::{Extraction, TryExtract},
};

/// Extracts the link label from the would be link reference definition segment.
///
/// Up to 3 spaces will be consumed first, then [LinkLabel::try_extract] will be called with
/// what comes next. If the call succeeds, this function will also test that the first character
/// in the remaining string is a colon (':'). Then, it will remove all following whitespace
/// until a non-whitespace character is found. If none are found, the remaining segment will
/// be empty.
pub fn try_extract_label(
    segment: LineSegment,
) -> Result<Extraction<LinkLabel, Segment>, LineSegment> {
    let mut start_offset = 0;
    let mut char_indices = segment.text().char_indices().take(3);
    while let Some(char_index) = char_indices.next() {
        if char_index.1 == ' ' {
            start_offset = char_index.0 + char_index.1.len_utf8();
        } else {
            break;
        }
    }
    let unindented = segment.slice(start_offset..segment.len());
    match LinkLabel::try_extract(unindented) {
        Ok(extraction) => {
            let label = extraction.extracted;

            // Make sure the first character in the remaining segment is a colon, otherwise it's an error.
            if !extraction.remaining.starts_with(":") {
                return Err(segment);
            }

            // Consume all whitespaces until a non-whitespace character is found.
            Ok(Extraction::new(
                label,
                extraction.remaining.slice(1..).trim_start(),
            ))
        }
        Err(_) => Err(segment),
    }
}

/// Extracts the link destination from the would be link reference definition segment.
///
/// If the segment is an invalid link destination, an error is returned. If a link destination
/// could be extracted, then the following whitespaces are removed from the returned remaining
/// segment.
pub fn try_extract_destination(
    segment: Segment,
) -> Result<Extraction<LinkDestination, Segment>, Segment> {
    match LinkDestination::try_extract(segment) {
        Ok(extraction) => Ok(Extraction::new(
            extraction.extracted,
            extraction.remaining.trim_start(),
        )),
        Err(_) => Err(segment),
    }
}

/// Extracts the title parser, or the title, from the would be link title.
///
/// If the segment is not valid as per the [LinkTitleParser]'s definition, then it is returned
/// as an error. Otherwise, the parser is returned if it is still expecting input, or the title
/// is returned if it has been successfully parsed.
pub fn try_parse_title(segment: Segment) -> Result<Either<LinkTitleParser, LinkTitle>, Segment> {
    let parser = LinkTitleParser::new();
    match parser.ingest(segment) {
        IngestResult::Ready(parser) => Ok(Either::Left(parser)),
        IngestResult::Success(link_title) => Ok(Either::Right(link_title)),
        IngestResult::Failure(_) => Err(segment),
    }
}
