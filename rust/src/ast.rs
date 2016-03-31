
pub enum Node {
    Step(Step),
    Background(Background),
    Scenario(Scenario),
    ScenarioOutline(ScenarioOutline),
    Tag(Tag),
    TableCell(TableCell),
    TableRow(TableRow),
    Examples(Examples),
    DataTable(DataTable),
    Comment(Comment),
    DocString(DocString),
}

pub enum ScenarioDefinition {
    Scenario(Scenario),
    ScenarioOutline(ScenarioOutline),
    Background(Background)
}

pub enum Argument {
    DataTable(DataTable),
    DocString(DocString),
}

pub struct Background {
    location: Location,
    keyword: String,
    name: String,
    description: String,
    steps: Vec<Step>
}

pub struct Comment {
    location: Location,
    text: String
}

pub struct DataTable {
    rows: Vec<TableRow>,
}

pub struct DocString {
    location: Location,
    content_type: String,
    content: String
}

pub struct Examples {
    location: Location,
    keyword: String,
    name: String,
    description: String,
    table_body: Vec<TableRow>,
    table_header: TableRow,
    tags: Vec<Tag>
}

pub struct Feature {
    location: Location,
    tags: Vec<Tag>,
    language: String,
    keyword: String,
    name: String,
    description: String,
    children: Vec<ScenarioDefinition>,
    comments: Vec<Comment>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    line: usize,
    column: usize
}

pub struct Scenario {
    location: Location,
    keyword: String,
    name: String,
    description: String,
    steps: Vec<Step>,
    tags: Vec<Tag>
}

pub struct ScenarioOutline {
    location: Location,
    keyword: String,
    name: String,
    description: String,
    steps: Vec<Step>,
    tags: Vec<Tag>,
    examples: Vec<Examples>
}

pub struct Step {
    location: Location,
    keyword: String,
    text: String,
    argument: Argument
}

pub struct TableCell {
    location: Location,
    value: String
}

pub struct TableRow {
    location: Location,
    cells: Vec<TableCell>
}


pub struct Tag {
    location: Location,
    name: String
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



impl Background {
    pub fn new(location: Location, keyword: String, name: String, description: String, steps: Vec<Step>) -> Background {
        Background {
            location: location,
            keyword: keyword,
            name: name,
            description: description,
            steps: steps
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }
}

impl Comment {
    pub fn new(location: Location, text: String) -> Comment {
        Comment {
            location: location,
            text: text
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl DataTable {
    pub fn new(rows: Vec<TableRow>) -> DataTable {
        DataTable {
            rows:rows
        }
    }


    pub fn get_rows(&self) -> &Vec<TableRow> {
        &self.rows
    }
}

impl DocString {
    pub fn new(location: Location, content_type: String, content: String) -> DocString {
        DocString {
            location: location,
            content: content,
            content_type: content_type
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_content_type(&self) -> &str {
        &self.content_type
    }
}

impl Examples {
    pub fn new(location: Location, tags: Vec<Tag>, keyword: String, name: String, description: String, table_header: TableRow, table_body: Vec<TableRow>) -> Examples {
        Examples {
            location: location,
            keyword: keyword,
            name: name,
            description: description,
            table_header: table_header,
            table_body: table_body,
            tags: tags
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_table_header(&self) -> &TableRow {
        &self.table_header
    }

    pub fn get_table_body(&self) -> &Vec<TableRow> {
        &self.table_body
    }
}

impl Feature {
    pub fn new(tags: Vec<Tag>, location: Location, language:String, keyword: String, name: String, description: String, children: Vec<ScenarioDefinition>, comments: Vec<Comment>) -> Feature {
        Feature {
            tags: tags,
            location: location,
            language: language,
            keyword: keyword,
            name: name,
            description: description,
            children: children,
            comments: comments
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn get_children(&self) -> &Vec<ScenarioDefinition> {
        &self.children
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }


}

impl Scenario {
    pub fn new(tags: Vec<Tag>, location: Location, keyword: String, name: String, description: String, steps: Vec<Step>) -> Scenario {
        Scenario {
            location: location,
            keyword: keyword,
            name: name,
            description: description,
            steps: steps,
            tags:tags
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }
}

impl ScenarioOutline {
    pub fn new(tags: Vec<Tag>, location: Location, keyword: String, name: String, description: String, steps: Vec<Step>, examples: Vec<Examples>) -> ScenarioOutline {
        ScenarioOutline {
            examples: examples,
            location: location,
            keyword: keyword,
            name: name,
            description: description,
            steps: steps,
            tags:tags
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_steps(&self) -> &Vec<Step> {
        &self.steps
    }

    pub fn get_examples(&self) -> &Vec<Examples> {
        &self.examples
    }
}

impl Step {
    pub fn new(location: Location, keyword: String, text: String, argument: Argument) -> Step {
        Step {
            location: location,
            keyword: keyword,
            text: text,
            argument: argument
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_keyword(&self) -> &str {
        &self.text
    }

    pub fn get_argument(&self) -> &Argument {
        &self.argument
    }
}

impl TableCell {
    pub fn new(location: Location, value: String) -> TableCell {
        TableCell {
            location: location,
            value: value
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl TableRow {
    pub fn new(location: Location, cells: Vec<TableCell>) -> TableRow {
        TableRow {
            location: location,
            cells: cells
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_cells(&self) -> &Vec<TableCell> {
        &self.cells
    }
}


impl Tag {
    pub fn new(location: Location, name: String) -> Tag {
        Tag {
            location: location,
            name: name
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
