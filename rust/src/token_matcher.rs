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
        println!("Scenario keywords {:?}", self.current_dialect.get_scenario_keywords());
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
                        println!("Switching language to {:?}", language);
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
                println!("{:?}", self.indent_to_remove);
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

    macro_rules! opt_to_string {
        ($incoming: expr) => {{
            let incoming: Option<&str> = $incoming;
            match incoming {
                Some(s) => Some(s.to_string()),
                None => None
            }
        }};
    }

    macro_rules! check_and_call {
        ($to_call: path, $this_variant: expr, $expected_type: expr, $token: expr, $matcher: expr, $expected_text: expr, $expected_items: expr, $expected_dialect: expr, $expected_indent: expr, $expected_location: expr) => {
            if $this_variant == $expected_type  {
                assert!($to_call($matcher, $token));
                assert_eq!($token.matched_type, $expected_type);
                assert_eq!($token.matched_text, opt_to_string!($expected_text));
                assert_eq!($token.matched_items, $expected_items);
                assert_eq!($token.matched_gherkin_dialect, $expected_dialect);
                assert_eq!($token.matched_indent, $expected_indent);
                assert_eq!($token.location, $expected_location);
                true
            } else {
                assert!(!$to_call($matcher, $token));
                false
            }
        };
    }

    fn check_token(scanner: &mut TokenScanner<Cursor<&[u8]>>, matcher: &mut TokenMatcher, expected_type: TokenType, expected_text: Option<&str>, expected_items: Option<Vec<GherkinLineSpan>>, expected_dialect: &GherkinDialect, expected_indent: usize, expected_location: Location) -> Token {
        let mut token = scanner.read();
        let mut has_matched = false;
        has_matched |= check_and_call!(TokenMatcher::match_EOF, TokenType::EOF, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_Empty, TokenType::Empty, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_TagLine, TokenType::TagLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_FeatureLine, TokenType::FeatureLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_BackgroundLine, TokenType::BackgroundLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_ScenarioLine, TokenType::ScenarioLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_ScenarioOutlineLine, TokenType::ScenarioOutlineLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_ExamplesLine, TokenType::ExamplesLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_StepLine, TokenType::StepLine, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_DocStringSeparator, TokenType::DocStringSeparator, expected_type, &mut token, matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= check_and_call!(TokenMatcher::match_TableRow, TokenType::TableRow, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        let matched_language =  check_and_call!(TokenMatcher::match_Language, TokenType::Language, expected_type, &mut token, matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        has_matched |= matched_language;
        // A language line looks just like a comment line, so the check for comment will incorrectly complain
        if !matched_language {
            has_matched |= check_and_call!(TokenMatcher::match_Comment, TokenType::Comment, expected_type, &mut token, &matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location);
        }
        if !has_matched {
            assert!(check_and_call!(TokenMatcher::match_Other, TokenType::Other, expected_type, &mut token, matcher, expected_text, expected_items, expected_dialect, expected_indent, expected_location));
        }
        token
    }



    use std::io::Cursor;
    use token_scanner::TokenScanner;
    use token::Token;
    use super::{TokenMatcher, DIALECT_PROVIDER};
    use parser::{TokenType, ITokenScanner, ITokenMatcher};
    use gherkin_dialect::GherkinDialect;
    use gherkin_line_span::GherkinLineSpan;
    use ast::Location;
    #[test]
    fn matching_file() {
        let test_file = r"
        Feature: An example
            This should just be a random line
            ```xml
            <doc>A doc string</doc>
            ```

        Background:
            When testing a token scanner

        # A normal scenario
        Scenario Outline: An example outline
            Given something <yeah>
            When something else <oh yeah>
            Then a thing <thing>

        Examples:
            | yeah | oh yeah | thing |
            |67    | 78      |3      |

        # language: af
        Situasie: An Afrikaans scenario
            Gegewe We've learned a new language
            En     We're describing stuff
            *      Like this
            Maar   We're oddly bilingual
        ";
        let cursor = Cursor::new(test_file.as_bytes());
        let mut scanner = TokenScanner::new(cursor);
        let mut matcher = TokenMatcher::new();
        let dialect = DIALECT_PROVIDER.get_default(Location::new(1, 0)).unwrap();
        let af_dialect = DIALECT_PROVIDER.get_dialect("af", Location::new(1, 0)).unwrap();
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, dialect, 0, Location::new(1, 1));
        check_token(&mut scanner, &mut matcher, TokenType::FeatureLine, Some("An example"), None, dialect, 8, Location::new(2, 9));
        // Descriptive lines after a feature declaration don't have their indents removed.
        check_token(&mut scanner, &mut matcher, TokenType::Other, Some("            This should just be a random line"), None, dialect, 12, Location::new(3, 13));
        check_token(&mut scanner, &mut matcher, TokenType::DocStringSeparator, Some("xml"), None, dialect, 12, Location::new(4, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Other, Some("<doc>A doc string</doc>"), None, dialect, 12, Location::new(5, 13));
        check_token(&mut scanner, &mut matcher, TokenType::DocStringSeparator, None, None, dialect, 12, Location::new(6, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, dialect, 0, Location::new(7, 1));
        check_token(&mut scanner, &mut matcher, TokenType::BackgroundLine, Some(""), None, dialect, 8, Location::new(8, 9));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("testing a token scanner"), None, dialect, 12, Location::new(9, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, dialect, 0, Location::new(10, 1));
        // Comments are also taken in their entirety, instead of just the part after the marker
        check_token(&mut scanner, &mut matcher, TokenType::Comment, Some("        # A normal scenario"), None, dialect, 0, Location::new(11, 1));
        check_token(&mut scanner, &mut matcher, TokenType::ScenarioOutlineLine, Some("An example outline"), None, dialect, 8, Location::new(12, 9));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("something <yeah>"), None, dialect, 12, Location::new(13, 13));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("something else <oh yeah>"), None, dialect, 12, Location::new(14, 13));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("a thing <thing>"), None, dialect, 12, Location::new(15, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, dialect, 0, Location::new(16, 1));
        check_token(&mut scanner, &mut matcher, TokenType::ExamplesLine, Some(""), None, dialect, 8, Location::new(17, 9));
        check_token(&mut scanner, &mut matcher, TokenType::TableRow, None, Some(vec![
            GherkinLineSpan::new(15, "yeah".to_string()) ,
            GherkinLineSpan::new(22, "oh yeah".to_string()),
            GherkinLineSpan::new(32, "thing".to_string())
        ]), dialect, 12, Location::new(18, 13));
        check_token(&mut scanner, &mut matcher, TokenType::TableRow, None, Some(vec![
            GherkinLineSpan::new(14, "67".to_string()) ,
            GherkinLineSpan::new(22, "78".to_string()),
            GherkinLineSpan::new(31, "3".to_string())
        ]), dialect, 12, Location::new(19, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, dialect, 0, Location::new(20, 1));
        check_token(&mut scanner, &mut matcher, TokenType::Language, Some("af"), None, dialect, 8, Location::new(21, 9));
        check_token(&mut scanner, &mut matcher, TokenType::ScenarioLine, Some("An Afrikaans scenario"), None, af_dialect, 8, Location::new(22, 9));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("We've learned a new language"), None, af_dialect, 12, Location::new(23, 13));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("We're describing stuff"), None, af_dialect, 12, Location::new(24, 13));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("Like this"), None, af_dialect, 12, Location::new(25, 13));
        check_token(&mut scanner, &mut matcher, TokenType::StepLine, Some("We're oddly bilingual"), None, af_dialect, 12, Location::new(26, 13));
        check_token(&mut scanner, &mut matcher, TokenType::Empty, None, None, af_dialect, 8, Location::new(27, 9));
        check_token(&mut scanner, &mut matcher, TokenType::EOF, None, None, af_dialect, 0, Location::new(28, 1));
    }
}
