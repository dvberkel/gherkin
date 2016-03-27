extern crate serde;
extern crate serde_json;
pub mod parser;
mod token;
mod gherkin_line;
mod gherkin_line_span;
mod gherkin_dialect;
mod ast;

const TITLE_KEYWORD_SEPARATOR:char = ':';

#[derive(Debug, PartialEq)]
pub struct ParserError {
    location: ast::Location,
    kind: ErrorKind
}
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    AstBuilder(String),
    ResourceNotFound(String),
    MalformedResource(String),
    NoSuchLanguage(String),
    UnexpectedToken(token::Token, Vec<String>, String),
    UnexpectedEOF(token::Token, Vec<String>, String),
    CompositeError(Vec<ParserError>)
}

impl ErrorKind {
    pub fn descripton(&self) -> String {
        match self {
            &ErrorKind::AstBuilder(ref message) => message.to_string(),
            &ErrorKind::ResourceNotFound(ref resource) => format!("Resource not found: {}", resource),
            &ErrorKind::MalformedResource(ref json_error) => format!("Malformed resource file: {}", json_error),
            &ErrorKind::NoSuchLanguage(ref language) => format!("No such language: {}", language),
            &ErrorKind::UnexpectedToken(ref token, ref expected, _) => format!("Unexpected token: {:?}.  Expected one of: {:?}", token, expected),
            &ErrorKind::UnexpectedEOF(_, ref expected, _) => format!("Unexpected EOF.  Expected one of: {:?}", expected),
            &ErrorKind::CompositeError(ref errors) => format!("{:?}", errors),
        }

    }
}

impl ParserError {
    pub fn new(kind:ErrorKind, location: ast::Location) -> ParserError {
        ParserError {
            kind: kind,
            location: location
        }
    }
}
