use crate::{ ir_l1, ir_l2 };

pub struct SvtnRow {
    s : String,
    v : String,
    t : String,
    n : String
}

impl SvtnRow {
    fn new(s : String, v : String, t : String, n : String) -> SvtnRow {
        SvtnRow {
            s,
            v,
            t,
            n
        }
    }

    fn to_vec(&self) -> Vec<String> {
        let s = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.s.clone()).unwrap()));
        let v = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.v.clone()).unwrap()));
        let t = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.t.clone()).unwrap()));
        let n = format!("{}", self.n);
        vec![s, v, t, n]
    }
}

pub struct SvtnTable {
    rows : Vec<SvtnRow>
}

impl SvtnTable {
    fn new(rows : Vec<SvtnRow>) -> SvtnTable {
        SvtnTable {
            rows
        }
    }

    pub fn parse(string : String) -> SvtnTable {
        let mut rows = vec![];
        for svtn in string.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().chunks(4) {
            if let [s, v, t, n] = svtn {
                rows.push(SvtnRow::new(s.to_string(), v.to_string(), t.to_string(), n.to_string()));
            } else {
                panic!("incomplete svtn table: {:?}", svtn);
            }
        }
        SvtnTable::new(rows)
    }
}

impl ir_l1::Component for SvtnTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let mut entries = vec![];
        entries.push(vec!["s".to_string(), "v".to_string(), "t".to_string(), "מסלול".to_string()]);
        for row in self.rows.iter() {
            entries.push(row.to_vec());
        }

        ir_l2_components.push(Box::new(ir_l2::Table::new(vec!["1in".to_string(), "1in".to_string(), "1in".to_string(), "1in".to_string()], entries, true)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

