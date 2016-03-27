use gherkin_dialect::{GherkinDialectProvider};
use token::Token;
use parser::{TokenType, ITokenMatcher};
use gherkin_line_span::GherkinLineSpan;
use ast::Location;
use COMMENT_PREFIX;

pub struct TokenMatcher {
    dialect_provider: GherkinDialectProvider,
    current_dialect: String,
    active_doc_string_separator: String,
    indent_to_remove: usize
}

impl TokenMatcher {
    pub fn new() -> TokenMatcher {
        TokenMatcher{
            current_dialect: "en".to_string(),
            dialect_provider: GherkinDialectProvider::new().unwrap(),
            active_doc_string_separator: String::new(),
            indent_to_remove: 0
        }
    }

    fn set_token_matched(&self, token: &mut Token, matched_type: TokenType, text: String, keyword: String, indent: Option<usize>, items: Vec<GherkinLineSpan>) {
        token.matched_type = matched_type;
        token.matched_keyword = keyword;
        token.matched_text = text;
        token.matched_items = items;
        token.matched_gherkin_dialect = self.current_dialect.clone();
        token.matched_indent = match indent {
            Some (indent) => indent,
            None =>
                match token.line {
                    Some(ref line) => line.indent(),
                    None => 0
            }
        };
        token.location = Location::new(token.location.get_line(), token.matched_indent + 1);
    }
}

impl ITokenMatcher for TokenMatcher {
    fn match_EOF(&self, token: &mut Token) -> bool {
        if token.is_eof() {
            self.set_token_matched(token, TokenType::EOF, String::new(), String::new(), None, Vec::new());
            true
        } else {
            false
        }
    }

    fn match_Empty(&self, token: &mut Token) -> bool {
        let is_empty = match token.line {
            Some(ref line) => line.is_empty(),
            None => false
        };
        if is_empty {
            self.set_token_matched(token, TokenType::Empty, String::new(), String::new(), None, Vec::new());
            true
        } else {
            false
        }
    }

    fn match_Comment(&self, token: &mut Token) -> bool {
        match token.line.clone() {
            Some(line) => if line.starts_with(COMMENT_PREFIX) {
                self.set_token_matched(token, TokenType::Comment, line.get_full_line_text(), String::new(), Some(0), Vec::new());
                true
            } else {
                false

            },
            None => false
        }

    }
    fn match_TagLine(&self, token: &mut Token) -> bool {
        true
    }
    fn match_FeatureLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_BackgroundLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_ScenarioLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_ScenarioOutlineLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_ExamplesLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_StepLine(&self, token: &mut Token) -> bool{
        true
    }
    fn match_DocStringSeparator(&self, token: &mut Token) -> bool{
        true
    }
    fn match_TableRow(&self, token: &mut Token) -> bool{
        true
    }
    fn match_Language(&self, token: &mut Token) -> bool{
        true
    }
    fn match_Other(&self, token: &mut Token) -> bool{
        true
    }

    fn reset(&mut self) {
        self.active_doc_string_separator = String::new();
        self.indent_to_remove = 0;
        self.current_dialect = "en".to_string();
    }
}
