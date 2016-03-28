use std::io::{BufRead, Lines};
use parser::{ITokenScanner};
use ast::Location;
use token::Token;
use gherkin_line::GherkinLine;

pub struct TokenScanner<R:BufRead> {
    line_number: usize,
    lines: Lines<R>
}

impl<R:BufRead> TokenScanner<R> {
    pub fn new(reader: R) -> TokenScanner<R> {
        TokenScanner {
            lines: reader.lines(),
            line_number: 0
        }
    }
}

impl<R: BufRead> ITokenScanner for TokenScanner<R> {
    fn read(&mut self) -> Token {
        let mut line = String::new();
        self.line_number += 1;
        let location = Location::new(self.line_number, 0);

        if let Some(line) = self.lines.next() {
            Token::new(Some(GherkinLine::new(line.unwrap(), self.line_number)), location)
        } else {
            Token::new(None, location)
        }

    }
}

#[cfg(test)]
mod test {
    use super::TokenScanner;
    use parser::ITokenScanner;
    use std::io::Cursor;
    use ast::Location;

    macro_rules! assert_line_eq {
        ($token:expr, $expected:expr) => ({
            let line = $token.line.unwrap();
            assert_eq!(line.get_rest_trimmed(0), $expected)
        });
    }

    #[test]
    fn read_lines() {
        let the_lines = "Given this stuff is working\n When I parse this\n  Then I should be happy";
        let the_lines = the_lines.as_bytes();
        let the_lines = Cursor::new(the_lines);
        let mut scanner = TokenScanner::new(the_lines);
        let given_token = scanner.read();
        assert_line_eq!(given_token, "Given this stuff is working");
        assert_eq!(given_token.location, Location::new(1, 0));
        let when_token = scanner.read();
        assert_line_eq!(when_token, "When I parse this");
        assert_eq!(when_token.location, Location::new(2, 0));
        let then_token = scanner.read();
        assert_line_eq!(then_token, "Then I should be happy");
        assert_eq!(then_token.location, Location::new(3, 0));
        let eof_token = scanner.read();
        assert!(eof_token.line.is_none());
    }
}
