use parser::TokenType;
use gherkin_line::GherkinLine;
use gherkin_line_span::GherkinLineSpan;
use ast::Location;
use std::fmt;

#[derive(PartialEq)]
pub struct Token {
    pub line: Option<GherkinLine>,
    pub matched_type: TokenType,
    pub matched_keyword: String,
    pub matched_text: String,
    pub matched_items: Vec<GherkinLineSpan>,
    pub matched_indent: usize,
    pub matched_gherkin_dialect: String,
    pub location: Location,
}

impl Token {
    pub fn new(line: Option<GherkinLine>, location:Location) -> Token {
        Token {
            line: line,
            location: location,
            matched_type: TokenType::None,
            matched_keyword: String::new(),
            matched_text: String::new(),
            matched_items: Vec::new(),
            matched_indent: 0,
            matched_gherkin_dialect: String::new()
        }
    }

    pub fn is_eof(&self) -> bool {
        self.line.is_some()
    }

    pub fn get_token_value(&self) -> Option<&str> {
        match self.line {
            Some(ref line) => Some(line.get_line_text(0)),
            None => None
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{:?}: {}{}", self.matched_type, self.matched_keyword, self.matched_text)
    }
}
