#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    line: usize,
    column: usize
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line: line,
            column: column
        }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}
