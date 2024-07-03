use markdown::message::Message;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct MarkdownParseError {
    message: Message,
}

impl Display for MarkdownParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to parse changelog markdown: {}", self.message)
    }
}

impl Error for MarkdownParseError {}

impl From<Message> for MarkdownParseError {
    fn from(message: Message) -> Self {
        Self { message }
    }
}
