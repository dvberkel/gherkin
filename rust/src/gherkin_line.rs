use TITLE_KEYWORD_SEPARATOR;
use gherkin_line_span::GherkinLineSpan;

#[derive(Debug, PartialEq)]
pub struct GherkinLine {
    line_number: u64,
    line_text: String,
    trimmed_line_text: String
}

impl GherkinLine {
    pub fn new(line_text: String, line_number: u64) -> GherkinLine {
        let trimmed_text = line_text.trim().to_string();
        GherkinLine {
            line_text: line_text,
            trimmed_line_text: trimmed_text,
            line_number: line_number
        }
    }

    /// Gets the line starting from `length` and strips off any remaining spaces.
    pub fn get_rest_trimmed(&self, length: usize) -> &str {
        self.trimmed_line_text.split_at(length).1.trim()
    }

    /// Gets the text of the line, after removing `indent_to_remove` spaces from the front.
    pub fn get_line_text(&self, indent_to_remove: usize) -> &str {
        if indent_to_remove > self.indent() {
            &self.line_text
        } else {
            self.line_text.split_at(indent_to_remove).1
        }
    }

    ///Checks if this is an empty line
    pub fn is_empty(&self) -> bool {
        self.trimmed_line_text.len() == 0
    }

    /// Checks if the line starts with a given `prefix`
    pub fn starts_with<'a>(&self, prefix: &'a str) -> bool {
        self.trimmed_line_text.starts_with(prefix)
    }

    /// Checks if the line starts with a given prefix followed by the title keyword separator
    pub fn starts_with_title_keyword<'a>(&self, prefix: &'a str)  -> bool {
        self.starts_with(prefix) &&
            self.trimmed_line_text.split_at(prefix.len()).1.starts_with(TITLE_KEYWORD_SEPARATOR)
    }

    fn indent(&self) -> usize {
        self.line_text.len() - self.trimmed_line_text.len()
    }

    pub fn get_table_cells(&self) -> Vec<GherkinLineSpan> {
        let mut line_spans = Vec::new();
        let mut cell: String = String::new();
        let mut before_first = true;
        let mut start_col = 0;
        let mut escaping = false;
        let indent = self.indent();
        for (col, c) in self.trimmed_line_text.chars().enumerate() {
            if escaping {
                match c {
                    'n' => {
                        cell.push('\n')
                    },
                    '\\'  => {
                        cell.push('\\')
                    },
                    '|' => {
                        cell.push('|')
                    },
                    _ => {
                        cell.push('\\');
                        cell.push(c)
                    }
                }
                escaping = false;
            } else {
                match c {
                    '|' => {
                        if before_first {
                            before_first = false;
                        } else {
                            let mut content_start = 0;
                            for content_char in cell.chars() {
                                if !content_char.is_whitespace() {
                                    break;
                                }
                                content_start += 1
                            }
                            if content_start == cell.len() {
                                content_start = 0;
                            }
                            line_spans.push(GherkinLineSpan::new(indent + start_col + content_start + 2, cell.trim().to_string()));
                            start_col = col;
                            cell = String::new();
                        }
                    },
                    '\\' => {
                        escaping = true;
                    },
                    _ => {
                        cell.push(c);
                    }
                }
            }
        }

        line_spans
    }


    pub fn get_tags(&self) -> Vec<GherkinLineSpan> {
        let mut line_spans = Vec::new();
        let mut column = self.indent() + 1;
        // TODO There is a bug here in this, the python and the dotnet implementation at least.
        // If there is more than one whitespace, this is not going to work
        for span in self.trimmed_line_text.split_whitespace() {
            line_spans.push(GherkinLineSpan::new(column, span.to_string()));
            column += span.len() + 1;
        }
        line_spans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gherkin_line_span::GherkinLineSpan;

    #[test]
    fn get_tags() {
        let line = GherkinLine::new("@billing @bicker @annoy".to_string(), 1);
        assert_eq!(line.get_tags(), vec![
            GherkinLineSpan::new(1, "@billing".to_string()),
            GherkinLineSpan::new(10, "@bicker".to_string()),
            GherkinLineSpan::new(18, "@annoy".to_string())
        ]);
    }

    #[test]
    fn get_table_cells() {
        let line = GherkinLine::new("   | name   | email            | twitter          |".to_string(), 1);
        assert_eq!(line.get_table_cells(), vec![
            GherkinLineSpan::new(6, "name".to_string()),
            GherkinLineSpan::new(15, "email".to_string()),
            GherkinLineSpan::new(34, "twitter".to_string())
        ]);

        let line = GherkinLine::new("| new\\nline | pipe \\| | slash \\\\ | other \\v|".to_string(), 1);
        assert_eq!(line.get_table_cells(), vec![
            GherkinLineSpan::new(3, "new\nline".to_string()),
            GherkinLineSpan::new(15, "pipe |".to_string()),
            GherkinLineSpan::new(25, "slash \\".to_string()),
            GherkinLineSpan::new(36, "other \\v".to_string())
        ]);
    }

}
