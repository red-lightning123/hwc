use crate::{ ir_l1, ir_l2 };

pub enum Un {
    UCurve,
    NCurve
}

pub struct UnRow {
    un : Un,
    statements : String
}

impl UnRow {
    fn new(un : Un, statements : String) -> UnRow {
        UnRow {
            un,
            statements
        }
    }

    fn to_vec(&self) -> Vec<String> {
        let un = match self.un {
            Un::UCurve => "\\( \\displaystyle \\cup \\)",
            Un::NCurve => "\\( \\displaystyle \\cap \\)"
        };
        let statements = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.statements.clone()).unwrap()));
        vec![statements, un.to_string()]
    }
}

pub struct UnTable {
    rows : Vec<UnRow>
}

impl UnTable {
    fn new(rows : Vec<UnRow>) -> UnTable {
        UnTable {
            rows
        }
    }
    
    pub fn parse(string : String) -> UnTable {
        let mut rows = vec![];
        for un_row in string.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let [un_text, statements] = un_row.split("#").map(|s| s.trim()).collect::<Vec<_>>()[..] {
                let un =
                    if un_text == "uc" {
                        Un::UCurve
                    } else if un_text == "nc" {
                        Un::NCurve
                    } else {
                        panic!("un table unrecognized un text: \"{}\"", un_text)
                    };
                rows.push(UnRow::new(un, statements.to_string()));
            } else {
                panic!("incomplete un table: {:?}", un_row);
            }
        }
        UnTable::new(rows)
    }
}

impl ir_l1::Component for UnTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let mut entries = vec![];
        entries.push(vec!["תחום".to_string(), "קעירות".to_string()]);
        for row in self.rows.iter() {
            entries.push(row.to_vec());
        }

        ir_l2_components.push(Box::new(ir_l2::Table::new(vec!["2in".to_string(), "2in".to_string()], entries, true)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

