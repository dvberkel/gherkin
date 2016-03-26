use parser::TokenType;
use gherkin_line::GherkinLine;
use gherkin_line_span::GherkinLineSpan;
use gherkin_dialect::GherkinDialect;
use ast::Location;

pub struct Token<'a> {
    pub line: GherkinLine,
    pub matchedType: TokenType,
    pub matchedKeyword: String,
    pub matchedText: String,
    pub matchedItems: Vec<GherkinLineSpan>,
    pub matchedIndent: i32,
    pub matchedGherkinDialect: &'a GherkinDialect,
    pub location: Location,
}
