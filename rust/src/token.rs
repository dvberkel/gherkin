use parser::TokenType;
use gherkin_line::GherkinLine;
use gherkin_line_span::GherkinLineSpan;
use gherkin_dialect::GherkinDialect;
use ast::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub line: GherkinLine,
    pub matchedType: TokenType,
    pub matchedKeyword: String,
    pub matchedText: String,
    pub matchedItems: Vec<GherkinLineSpan>,
    pub matchedIndent: i32,
    pub matchedGherkinDialect: String,
    pub location: Location,
}
