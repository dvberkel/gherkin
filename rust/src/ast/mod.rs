#[derive(Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line: line,
            column: column
        }
    }
}
