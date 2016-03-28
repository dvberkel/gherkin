use parser::TokenType;
use gherkin_line::GherkinLine;
use gherkin_line_span::GherkinLineSpan;
use gherkin_dialect::{GherkinDialect};
use token_matcher::DIALECT_PROVIDER;
use ast::Location;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub line: Option<GherkinLine>,
    pub matched_type: TokenType,
    pub matched_keyword: Option<String>,
    pub matched_text: Option<String>,
    pub matched_items: Option<Vec<GherkinLineSpan>>,
    pub matched_indent: usize,
    pub matched_gherkin_dialect: &'static GherkinDialect,
    pub location: Location,
}

impl Token {
    pub fn new(line: Option<GherkinLine>, location:Location) -> Token {
        Token {
            line: line,
            location: location.clone(),
            matched_type: TokenType::None,
            matched_keyword: None,
            matched_items: None,
            matched_text: None,
            matched_indent: 0,
            matched_gherkin_dialect: DIALECT_PROVIDER.get_default(location).unwrap()
        }
    }

    pub fn is_eof(&self) -> bool {
        self.line.is_none()
    }

    pub fn get_token_value(&self) -> Option<&str> {
        match self.line {
            Some(ref line) => Some(line.get_line_text(0)),
            None => None
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let empty = "".to_string();
        let keyword = match self.matched_keyword {
            Some(ref keyword) => keyword,
            None => &empty
        };
        let text = match self.matched_text {
            Some(ref text) => text,
            None => &empty
        };
        write!(formatter, "{:?}: {}{}", self.matched_type, keyword, text)
    }
}
