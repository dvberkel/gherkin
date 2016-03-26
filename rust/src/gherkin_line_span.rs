
#[derive(Debug, PartialEq)]
pub struct GherkinLineSpan {
    column: usize,
    text: String,
}

impl GherkinLineSpan {

    #[inline]
    pub fn new(column: usize, text: String) -> GherkinLineSpan {
        GherkinLineSpan {
            column: column,
            text: text
        }
    }
}
