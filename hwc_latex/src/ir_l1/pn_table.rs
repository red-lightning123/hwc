use crate::{ ir_l1, ir_l2 };

pub enum Pn {
    Positive,
    Negative
}

pub struct PnRow {
    pn : Pn,
    statements : String
}

impl PnRow {
    fn new(pn : Pn, statements : String) -> PnRow {
        PnRow {
            pn,
            statements
        }
    }

    fn to_vec(&self) -> Vec<String> {
        let pn = match self.pn {
            Pn::Positive => "חיובית",
            Pn::Negative => "שלילית"
        };
        let statements = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.statements.clone()).unwrap()));
        vec![statements, pn.to_string()]
    }
}

pub struct PnTable {
    rows : Vec<PnRow>
}

impl PnTable {
    fn new(rows : Vec<PnRow>) -> PnTable {
        PnTable {
            rows
        }
    }

    pub fn parse(string : String) -> PnTable {
        let mut rows = vec![];
        for pn_row in string.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let [pn_text, statements] = pn_row.split("#").map(|s| s.trim()).collect::<Vec<_>>()[..] {
                let pn =
                    if pn_text == "p" {
                        Pn::Positive
                    } else if pn_text == "n" {
                        Pn::Negative
                    } else {
                        panic!("pn table unrecognized pn text: \"{}\"", pn_text)
                    };
                rows.push(PnRow::new(pn, statements.to_string()));
            } else {
                panic!("incomplete pn table: {:?}", pn_row);
            }
        }
        PnTable::new(rows)
    }
}

impl ir_l1::Component for PnTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let mut entries = vec![];
        entries.push(vec!["תחום".to_string(), "חיוביות/שליליות".to_string()]);
        for row in self.rows.iter() {
            entries.push(row.to_vec());
        }

        ir_l2_components.push(Box::new(ir_l2::Table::new(vec!["2in".to_string(), "2in".to_string()], entries, true)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

