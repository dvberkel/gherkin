use gherkin_dialect::{GherkinDialectProvider, GherkinDialect};
use token::Token;
use parser::{TokenType, ITokenMatcher};
use gherkin_line_span::GherkinLineSpan;
use ast::Location;
use {TAG_PREFIX, COMMENT_PREFIX, TITLE_KEYWORD_SEPARATOR, DOCSTRING_SEPARATOR, DOCSTRING_ALTERNATIVE_SEPARATOR, TABLE_CELL_SEPARATOR};
use regex::Regex;

lazy_static! {
    pub static ref DIALECT_PROVIDER: GherkinDialectProvider =  GherkinDialectProvider::new().unwrap();
    static ref LANGUAGE_PATTERN: Regex = Regex::new("^\\s*#\\s*language\\s*:\\s*([a-zA-Z-_]+)\\s*$").unwrap();
}

pub struct TokenMatcher {
    current_dialect: &'static GherkinDialect,
    active_doc_string_separator: Option<&'static str>,
    indent_to_remove: usize
}

impl TokenMatcher {
    pub fn new() -> TokenMatcher {

        TokenMatcher{
            current_dialect: DIALECT_PROVIDER.get_default(Location::new(1, 0)).unwrap(),
            active_doc_string_separator: None,
            indent_to_remove: 0
        }
    }

    fn set_token_matched(&self, token: &mut Token, matched_type: TokenType, text: Option<String>, keyword: Option<String>, indent: Option<usize>, items: Option<Vec<GherkinLineSpan>>) {
        token.matched_type = matched_type;
        token.matched_keyword = keyword;
        token.matched_text = text;
        token.matched_items = items;
        token.matched_gherkin_dialect = self.current_dialect;
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

    #[allow(non_snake_case)]
    fn match_TitleLine(&self, token: &mut Token, token_type: TokenType, keywords: &Vec<String>) -> bool {
        if token.line.is_none() {
            return false;
        }
        let line = token.line.clone().unwrap();
        for keyword in keywords.iter() {
            if line.starts_with_title_keyword(keyword) {
                let title = line.get_rest_trimmed(keyword.len() + TITLE_KEYWORD_SEPARATOR.len());
                self.set_token_matched(token, token_type, Some(title.to_string()), Some(keyword.to_string()), None, None);
                return true
            }
        }
        false
    }

    fn match_specific_doc_string_separator(&mut self, token: &mut Token, separator: &'static str , is_open: bool ) -> bool {
        if token.line.is_none() {
            return false;
        }
        let line = token.line.clone().unwrap();
        if line.starts_with(separator) {
            if is_open {
                let content_type = line.get_rest_trimmed(separator.len());
                self.active_doc_string_separator = Some(separator);
                self.indent_to_remove = line.indent();
                self.set_token_matched(token, TokenType::DocStringSeparator, Some(content_type.to_string()), None, None, None);
            } else {
                self.active_doc_string_separator = None;
                self.indent_to_remove = 0;
                self.set_token_matched(token, TokenType::DocStringSeparator, None, None, None, None);
            }
            true
        } else {
            false
        }
    }

    fn unescape_doc_string(&self, text: &str) -> String {
        match self.active_doc_string_separator {
            Some(_) => text.replace("\\\"\\\"\\\"", "\"\"\""),
            None => text.to_string()
        }
    }
}

impl ITokenMatcher for TokenMatcher {
    fn match_EOF(&self, token: &mut Token) -> bool {
        if token.is_eof() {
            self.set_token_matched(token, TokenType::EOF, None, None, None, None);
            true
        } else {
            false
        }
    }

    fn match_Empty(&self, token: &mut Token) -> bool {
        match token.line.clone() {
            Some(ref line) => if line.is_empty() {
                self.set_token_matched(token, TokenType::Empty, None, None, None, None);
                true
            } else {
                false
            },
            None => false
        }

    }

    fn match_Comment(&self, token: &mut Token) -> bool {
        match token.line.clone() {
            Some(line) => if line.starts_with(COMMENT_PREFIX) {
                self.set_token_matched(token, TokenType::Comment, Some(line.get_full_line_text()), None, Some(0), None);
                true
            } else {
                false

            },
            None => false
        }

    }
    fn match_TagLine(&self, token: &mut Token) -> bool {
        match token.line.clone() {
            Some(line) => if line.starts_with(TAG_PREFIX) {
                let tags = line.get_tags();
                self.set_token_matched(token, TokenType::TagLine, None, None, None, Some(tags));
                true
            } else {
                false

            },
            None => false
        }
    }
    fn match_FeatureLine(&self, token: &mut Token) -> bool{
        self.match_TitleLine(token, TokenType::FeatureLine, self.current_dialect.get_feature_keywords())
    }
    fn match_BackgroundLine(&self, token: &mut Token) -> bool{
        self.match_TitleLine(token, TokenType::BackgroundLine, self.current_dialect.get_background_keywords())
    }
    fn match_ScenarioLine(&self, token: &mut Token) -> bool{
        self.match_TitleLine(token, TokenType::ScenarioLine, self.current_dialect.get_scenario_keywords())
    }
    fn match_ScenarioOutlineLine(&self, token: &mut Token) -> bool{
        self.match_TitleLine(token, TokenType::ScenarioOutlineLine, self.current_dialect.get_scenario_outline_keywords())
    }
    fn match_ExamplesLine(&self, token: &mut Token) -> bool{
        self.match_TitleLine(token, TokenType::ExamplesLine, self.current_dialect.get_examples_keywords())
    }
    fn match_StepLine(&self, token: &mut Token) -> bool{
        if token.line.is_none() {
            return false;
        }
        let line = token.line.clone().unwrap();
        for keyword in self.current_dialect.get_step_keywords().iter() {
            if line.starts_with(keyword) {
                let step_text = line.get_rest_trimmed(keyword.len());
                self.set_token_matched(token, TokenType::StepLine, Some(step_text.to_string()), Some(keyword.to_string()), None, None);
                return true
            }
        }
        false
    }
    fn match_DocStringSeparator(&mut self, token: &mut Token) -> bool{
        match self.active_doc_string_separator {
            None  => self.match_specific_doc_string_separator(token, DOCSTRING_SEPARATOR, true) ||
                     self.match_specific_doc_string_separator(token, DOCSTRING_ALTERNATIVE_SEPARATOR, true),
            Some(separator) => {
                self.match_specific_doc_string_separator(token, separator, false)

            }
        }
    }
    fn match_TableRow(&self, token: &mut Token) -> bool{
        match token.line.clone() {
            Some(line) => if line.starts_with(TABLE_CELL_SEPARATOR) {
                let cells = line.get_table_cells();
                self.set_token_matched(token, TokenType::TableRow, None, None, None, Some(cells));
                true
            } else {
                false

            },
            None => false
        }
    }
    fn match_Language(&mut self, token: &mut Token) -> bool{
        match token.line.clone() {
            Some(line) =>
                match LANGUAGE_PATTERN.captures(line.get_line_text(0)) {
                    Some(matcher) => {
                        let language = matcher.at(1).unwrap();
                        self.set_token_matched(token, TokenType::Language, Some(language.to_string()), None, None, None);
                        self.current_dialect = DIALECT_PROVIDER.get_dialect(language, token.location.clone()).unwrap();
                        true
                    },
                    None => false
                },
                None => false

        }
    }
    fn match_Other(&self, token: &mut Token) -> bool{
        match token.line.clone() {
            Some(line) => {
                let text = line.get_line_text(self.indent_to_remove);
                self.set_token_matched(token, TokenType::Other, Some(self.unescape_doc_string(text)), None, None, None);
                true
            },
            None => false
        }
    }

    fn reset(&mut self) {
        self.active_doc_string_separator = None;
        self.indent_to_remove = 0;
        self.current_dialect = DIALECT_PROVIDER.get_default(Location::new(1, 0)).unwrap();
    }
}

#[cfg(test)]
mod test {
    macro_rules! check_and_call {
        ($to_call: path, $this_variant: expr, $expected_type: expr, $token: expr, $matcher: expr, $expected_text: expr, $expected_items: expr, $expected_dialect: expr, $expected_indent: expr, $expected_location: expr) => {
            if $this_variant == $expected_type  {
                assert!($to_call($matcher, $token));
                assert_eq!($token.matched_type, $expected_type);
                assert_eq!($token.matched_text, $expected_text);
                assert_eq!($token.matched_items, $expected_items);
                assert_eq!($token.matched_gherkin_dialect, $expected_dialect);
                assert_eq!($token.matched_indent, $expected_indent);
                assert_eq!($token.location, $expected_location);
            } else {
                assert!(!$to_call($matcher, $token))
            }
        };
    }

    macro_rules! check_token {
        ($scanner: expr, $matcher: expr, $expected_type: expr, $expected_text: expr, $expected_items: expr, $expected_dialect: expr, $expected_indent: expr, $expected_location: expr) => {
            {
                let mut token = $scanner.read();
                check_and_call!(TokenMatcher::match_EOF, TokenType::EOF, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_Empty, TokenType::Empty, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_Comment, TokenType::Comment, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_TagLine, TokenType::TagLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_FeatureLine, TokenType::FeatureLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_BackgroundLine, TokenType::BackgroundLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_ScenarioLine, TokenType::ScenarioLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_ScenarioOutlineLine, TokenType::ScenarioOutlineLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_ExamplesLine, TokenType::ExamplesLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_StepLine, TokenType::StepLine, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_DocStringSeparator, TokenType::DocStringSeparator, $expected_type, &mut token, &mut $matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_TableRow, TokenType::TableRow, $expected_type, &mut token, &$matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
                check_and_call!(TokenMatcher::match_Language, TokenType::Language, $expected_type, &mut token, &mut $matcher, $expected_text, $expected_items, $expected_dialect, $expected_indent, $expected_location);
            }
        };
    }

    use std::io::Cursor;
    use token_scanner::TokenScanner;
    use super::{TokenMatcher, DIALECT_PROVIDER};
    use parser::{TokenType, ITokenScanner, ITokenMatcher};
    use ast::Location;
    #[test]
    fn test_name() {
        let test_file = r"
        Given something
        When something else
        Then a thing
        ";
        let cursor = Cursor::new(test_file.as_bytes());
        let mut scanner = TokenScanner::new(cursor);
        let mut matcher = TokenMatcher::new();
        let dialect = DIALECT_PROVIDER.get_default(Location::new(1, 0)).unwrap();
        check_token!(scanner, matcher, TokenType::Empty, None, None, dialect, 0, Location::new(1, 1));
        check_token!(scanner, matcher, TokenType::StepLine, Some("something".to_string()), None, dialect, 8, Location::new(2, 9));
        check_token!(scanner, matcher, TokenType::StepLine, Some("something else".to_string()), None, dialect, 8, Location::new(3, 9));
        check_token!(scanner, matcher, TokenType::StepLine, Some("a thing".to_string()), None, dialect, 8, Location::new(4, 9));
    }
}
