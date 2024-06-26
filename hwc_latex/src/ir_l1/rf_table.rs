use crate::{ ir_l1, ir_l2 };

pub enum Rf {
    Rise,
    Fall,
    Max,
    Min,
    EdgeMax,
    EdgeMin,
    Inflection
}

pub struct RfRow {
    rf : Rf,
    statements : String
}

impl RfRow {
    fn new(rf : Rf, statements : String) -> RfRow {
        RfRow {
            rf,
            statements
        }
    }

    fn to_vec(&self) -> Vec<String> {
        let rf = match self.rf {
            Rf::Rise => "עלייה",
            Rf::Fall => "ירידה",
            Rf::Max => "מקסימום",
            Rf::Min => "מינימום",
            Rf::EdgeMax => "מקסימום קצה",
            Rf::EdgeMin => "מינימום קצה",
            Rf::Inflection => "פיתול"
        };
        let statements = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.statements.clone()).unwrap()));
        vec![statements, rf.to_string()]
    }
}

pub struct RfTable {
    rows : Vec<RfRow>
}

impl RfTable {
    fn new(rows : Vec<RfRow>) -> RfTable {
        RfTable {
            rows
        }
    }

    pub fn parse(string : String) -> RfTable {
        let mut rows = vec![];
        for rf_row in string.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let [rf_text, statements] = rf_row.split("#").map(|s| s.trim()).collect::<Vec<_>>()[..] {
                let rf =
                    if rf_text == "r" {
                        Rf::Rise
                    } else if rf_text == "f" {
                        Rf::Fall
                    } else if rf_text == "max" {
                        Rf::Max
                    } else if rf_text == "min" {
                        Rf::Min
                    } else if rf_text == "emax" {
                        Rf::EdgeMax
                    } else if rf_text == "emin" {
                        Rf::EdgeMin
                    } else if rf_text == "i" {
                        Rf::Inflection
                    } else {
                        panic!("rf table unrecognized rf text: \"{}\"", rf_text)
                    };
                rows.push(RfRow::new(rf, statements.to_string()));
            } else {
                panic!("incomplete rf table: {:?}", rf_row);
            }
        }
        RfTable::new(rows)
    }
}

impl ir_l1::Component for RfTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let mut entries = vec![];
        entries.push(vec!["תחום".to_string(), "עלייה/ירידה".to_string()]);
        for row in self.rows.iter() {
            entries.push(row.to_vec());
        }

        ir_l2_components.push(Box::new(ir_l2::Table::new(vec!["2in".to_string(), "2in".to_string()], entries, true)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

