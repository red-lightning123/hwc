pub enum WriteDirection {
    LTR,
    RTL
}

pub struct DocumentWriteState {
    pub direction : WriteDirection,
    pub at_start_of_line : bool
}

impl DocumentWriteState {
    fn new() -> DocumentWriteState {
        DocumentWriteState {
            direction : WriteDirection::RTL,
            at_start_of_line : true
        }
    }
}

pub trait Component {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String;
}

pub struct Document {
    title : String,
    author : String,
    date : String,
    components : Vec<Box<dyn Component>>
}

impl Document {
    pub fn new(title : String, author : String, date : String, components : Vec<Box<dyn Component>>) -> Document {
        Document {
            title,
            author,
            date,
            components
        }
    }

    pub fn write_latex(&self) -> String {
        let mut s = String::new();
        s += include_str!("doc_begin.latex");
        s += &format!("\\title{{{}}}\n", self.title);
        s += &format!("\\author{{{}}}\n", self.author);
        s += &format!("\\date{{{}}}\n", self.date);
        s += "\\begin{document}\n";

        let mut state = DocumentWriteState::new();
        
        for component in self.components.iter() {
            s += &component.write_latex(&mut state);
        }

        s += "\\end{document}\n";

        s
    }
}

pub struct Intro;

impl Intro {
    pub fn new() -> Intro {
        Intro
    }
}

impl Component for Intro {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\maketitle\n";
        s
    }
}

pub struct Section {
    name : String
}

impl Section {
    pub fn new(name : String) -> Section {
        Section {
            name
        }
    }
}

impl Component for Section {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\section{";
        s += &self.name;
        s += "}\n";
        s
    }
}

pub struct Subsection {
    name : String
}

impl Subsection {
    pub fn new(name : String) -> Subsection {
        Subsection {
            name
        }
    }
}

impl Component for Subsection {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\subsection{";
        s += &self.name;
        s += "}\n";
        s
    }
}

pub struct Subsubsection {
    name : String
}

impl Subsubsection {
    pub fn new(name : String) -> Subsubsection {
        Subsubsection {
            name
        }
    }
}

impl Component for Subsubsection {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\subsubsection{";
        s += &self.name;
        s += "}\n";
        s
    }
}

pub struct NewLine;

impl NewLine {
    pub fn new() -> NewLine {
        NewLine
    }
}

impl Component for NewLine {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        state.at_start_of_line = true;
        s += " \\\\\n";
        s
    }
}

pub struct NewPage;

impl NewPage {
    pub fn new() -> NewPage {
        NewPage
    }
}

impl Component for NewPage {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        state.at_start_of_line = true;
        s += "\\newpage\n";
        s
    }
}

pub struct InlineText {
    text : String
}

impl InlineText {
    pub fn new(text : String) -> InlineText {
        InlineText {
            text
        }
    }
}

impl Component for InlineText {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            if state.at_start_of_line {
                s += "\\RTL\n";
                state.direction = WriteDirection::RTL;
            }
        }
        state.at_start_of_line = false;
        s += &self.text;
        //s += "\n";
        s
    }
}

pub struct InlineMath {
    statements : hwc_lang_equation::format::Statements
}

impl InlineMath {
    pub fn new(statements : hwc_lang_equation::format::Statements) -> InlineMath {
        InlineMath {
            statements
        }
    }
    pub fn parse(math : String) -> Result<InlineMath, String> {
        Ok(InlineMath::new(hwc_lang_equation::parse_statements(math)?))
    }
}

impl Component for InlineMath {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::RTL = state.direction {
            if state.at_start_of_line {
                s += "\\LTR\n";
                state.direction = WriteDirection::LTR;
            }
        }
        state.at_start_of_line = false;
        s += "\\( \\displaystyle  ";
        s += &crate::latexify::latexify_statements(&self.statements);
        s += "\\)";
        s
    }
}

pub struct Table {
    column_widths : Vec<String>,
    entries : Vec<Vec<String>>,
    flip_h : bool
}

impl Table {
    pub fn new(column_widths : Vec<String>, entries : Vec<Vec<String>>, flip_h : bool) -> Table {
        Table {
            column_widths,
            entries,
            flip_h
        }
    }
}

impl Component for Table {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\begin{longtable}{";

        s += "|";
        for column_width in &self.column_widths {
            s += "M{";
            s += &column_width;
            s += "}|";
        }

        s += "}\n";
        s += "\\hline\n";

        let flipped_entries = self.entries.iter().map(|entry| entry.iter().rev().map(|s| s.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let entries = if self.flip_h {
            &flipped_entries
        } else {
            &self.entries
        };

        for row in entries {
            if row.is_empty() {
                panic!("table has to have at least one column")
            }
            s += &row[0];
            for entry in &row[1..] {
                s += " & ";
                s += entry;
            }
            s += " \\\\\n";
            s += "\\hline\n";
        }
        s += "\\end{longtable}\n";
        s
    }
}

pub fn latexify_table_math_text_entry(string : String) -> Result<String, String> {
    let entry = hwc_lang_dollar::parse_file(string)?;
    let mut latex = String::new();
    for component in entry.components() {
        latex += &match_entry_math_text_component_to_latex(component)?;
    }
    Ok(latex)
}

fn match_entry_math_text_component_to_latex(component : &hwc_lang_dollar::format::Component) -> Result<String, String> {
    Ok(
        match component {
            hwc_lang_dollar::format::Component::InlineText(text) => {
                text.to_string()
            }
            hwc_lang_dollar::format::Component::InlineMath(math) => {
                format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(math.to_string())?))
            }
            hwc_lang_dollar::format::Component::MultilineMath(math) => {
                let mut s = String::new();
                s += "\\begin{tabular}{c} ";
                let multiline_statements = hwc_lang_equation::parse_multiline_statements(math.to_string())?;
                for item in multiline_statements.items() {
                    match item {
                        hwc_lang_equation::format::MultilineStatementsItem::Statements(statements) => {
                            s += &format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(statements));
                        },
                        hwc_lang_equation::format::MultilineStatementsItem::Newline(_newline) => {
                            s += " \\\\ ";
                        }
                    }
                }
                s += "\\end{tabular} ";
                s
            }
            hwc_lang_dollar::format::Component::Newline => {
                panic!("unimplemented handling of newlines in table")
            }
        }
    )
}

pub struct Image {
    path : String,
    width : String
}

impl Image {
    pub fn new(path : String, width : String) -> Image {
        Image {
            path,
            width
        }
    }
}

impl Component for Image {
    fn write_latex(&self, state : &mut DocumentWriteState) -> String {
        let mut s = String::new();
        if let WriteDirection::LTR = state.direction {
            s += "\\RTL\n";
            state.direction = WriteDirection::RTL;
        }
        state.at_start_of_line = true;
        s += "\\includegraphics[width=";
        s += &self.width;
        s += "]{";
        s += &self.path;
        s += "}\n";
        s
    }
}
