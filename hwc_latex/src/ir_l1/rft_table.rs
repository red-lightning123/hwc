use crate::{ ir_l1, ir_l2 };

/*pub enum Rft {
    Rise,
    Fall,
    Undefined,
    Max,
    Min,
    EdgeMax,
    EdgeMin,
    Inflection
}

pub struct RftRow {
    pub range_statements : String,
    pub value : String,
    pub rft : Rft
}

impl RftRow {
    fn new(range_statements : String, value : String, rft : Rft) -> RftRow {
        RftRow {
            range_statements,
            value,
            rft
        }
    }

    fn to_vec(&self) -> Vec<String> {
        let rft = match self.rft {
            Pnt::Positive => "חיובית",
            Pnt::Negative => "שלילית",
            Pnt::ZeroPoint => "חיתוך עם ציר \\( \\displaystyle x \\)",
            Pnt::Undefined => "לא מוגדרת",
        };
        let value = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.value.clone()).unwrap()));
        let range_statements = format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.range_statements.clone()).unwrap()));
        vec![rft.to_string(), value, range_statements]
    }
}*/

pub enum RangeType {
    Rise,
    Fall,
    Undefined
}

impl RangeType {
    fn sign_translated(&self) -> &str {
        match self {
            RangeType::Rise => "\\( \\displaystyle + \\)",
            RangeType::Fall => "\\( \\displaystyle - \\)",
            RangeType::Undefined => "\\BigCross"
        }
    }
    fn rf_translated(&self) -> &str {
        match self {
            RangeType::Rise => "\\( \\displaystyle \\nearrow \\)",
            RangeType::Fall => "\\( \\displaystyle \\searrow \\)",
            RangeType::Undefined => "\\BigCross"
        }
    }
}

impl TryFrom<&str> for RangeType {
    type Error = ();
    fn try_from(range_type : &str) -> Result<Self, Self::Error> {
        if range_type == "+" {
            Ok(RangeType::Rise)
        } else if range_type == "-" {
            Ok(RangeType::Fall)
        } else if range_type == "u" {
            Ok(RangeType::Undefined)
        } else {
            Err(())
        }
    }
}

pub enum PointType {
    Max,
    Min,
    EdgeMax,
    EdgeMin,
    Inflection,
    Undefined
}

impl PointType {
    fn sign_translated(&self) -> &str {
        match self {
            PointType::Undefined => "\\BigCross",
            _ => ""
        }
    }
    fn rf_translated(&self) -> &str {
        match self {
            PointType::Max => "\\( \\displaystyle max \\)",
            PointType::Min => "\\( \\displaystyle min \\)",
            PointType::EdgeMax => "\\( \\displaystyle max \\) קצה",
            PointType::EdgeMin => "\\( \\displaystyle min \\) קצה",
            PointType::Inflection => "פיתול",
            PointType::Undefined => "\\BigCross"
        }
    }
}

impl TryFrom<&str> for PointType {
    type Error = ();
    fn try_from(point_type : &str) -> Result<Self, Self::Error> {
        if point_type == "max" {
            Ok(PointType::Max)
        } else if point_type == "min" {
            Ok(PointType::Min)
        } else if point_type == "emax" {
            Ok(PointType::EdgeMax)
        } else if point_type == "emin" {
            Ok(PointType::EdgeMin)
        } else if point_type == "i" {
            Ok(PointType::Inflection)
        } else if point_type == "u" {
            Ok(PointType::Undefined)
        } else {
            Err(())
        }
    }
}

pub struct RftTable {
    range_cols : Vec<RangeType>,
    point_cols : Vec<(String, PointType)>
}

impl RftTable {
    fn new(range_cols : Vec<RangeType>, point_cols : Vec<(String, PointType)>) -> RftTable {
        RftTable {
            range_cols,
            point_cols
        }
    }

    pub fn parse(string : String) -> RftTable {
        let mut range_cols = vec![];
        let mut point_cols = vec![];
        for col_pair in string.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().chunks(2) {
            if let [range_col, point_col] = col_pair {
                range_cols.push(RangeType::try_from(*range_col).expect("RftTable::parse - invalid range type"));
                if let [point_pos] = point_col.split("#").map(|s| s.trim()).collect::<Vec<_>>()[..] {
                    point_cols.push((point_pos.to_string(), None));
                } else if let [point_pos, point_type] = point_col.split("#").map(|s| s.trim()).collect::<Vec<_>>()[..] {
                    point_cols.push((point_pos.to_string(), Some(PointType::try_from(point_type).expect("RftTable::parse - invalid point type"))));
                } else {
                    panic!("RftTable::parse - point col should have either 1 or 2 items");
                }
            } else if let [range_col] = col_pair {
                range_cols.push(RangeType::try_from(*range_col).expect("RftTable::parse - invalid range type"));
            } else {
                unreachable!();
            }
        }
        if range_cols.len() != point_cols.len() + 1 {
            panic!("RftTable::parse - table range and point cols do not match in numbers");
        }
        let mut point_cols_inferred = vec![];
        for point_col in point_cols {
            point_cols_inferred.push(
                (
                    point_col.0,
                    match point_col.1 {
                        Some(point_col) => point_col,
                        None => panic!("RftTable::parse - no implementation for inferring point types")
                    }
                )
            );
        }
        /*if () {
            cols.push(("\\( \\displaystyle x < \\)", ));
        }
        for (i, rft_row) in .enumerate() {
            let mut rft_row_it = rft_row.split("#").map(|s| s.trim());
            let mut range_statements =
                if let Some(range_statements) = rft_row_it.next() {
                    range_statements
                } else {
                    panic!("RftTable::parse - entry missing range statements")
                };

            /*if i % 2 == 0 {
            } else {
                if let Some(x) = rft_row_it.next() {
                    cols.push()
                }
            }*/

            if i % 2 == 0 {
                let mut value =
                    if let Some(value) = rft_row_it.next() {
                        value
                    } else {
                        panic!("RftTable::parse - entry missing value")
                    };
                let rft =
                    if let Ok(val) = value.parse::<f32>() {
                        if val > 0.0 {
                            Pnt::Positive
                        } else if val < 0.0 {
                            Pnt::Negative
                        } else {
                            panic!("RftTable::parse - range rft can't be zero")
                        }
                    } else if let Some(explicit_rft) = rft_row_it.next() {
                        if explicit_rft == "undef" {
                            value = "undef";
                            Pnt::Undefined
                        } else {
                            if explicit_rft == "p" {
                                Pnt::Positive
                            } else if explicit_rft == "n" {
                                Pnt::Negative
                            } else {
                                panic!("RftTable::parse - unrecognized rft string: {}", explicit_rft)
                            }
                        }
                    } else {
                        panic!("RftTable::parse - couldn't infer entry pnt")
                    };
                rows.push(RftRow::new(range_statements.to_string(), value.to_string(), pnt));
            } else {
                if let Some(value) = rft_row_it.next() {
                    if !value.is_empty() {
                        panic!("RftTable::parse - critical point entry can't have explicit value")
                    }
                }
                if let Some(explicit_rft) = pnt_row_it.next() {
                    if explicit_rft == "0" {
                        rows.push(RftRow::new(range_statements.to_string(), explicit_rft.to_string(), Pnt::ZeroPoint));
                    } else if explicit_rft == "undef" {
                        rows.push(RftRow::new(range_statements.to_string(), explicit_rft.to_string(), Pnt::Undefined));
                    } else {
                        panic!("RftTable::parse - unrecognized critical point entry explicit rft: {}", explicit_pnt)
                    }
                } else {
                    rows.push(RftRow::new(range_statements.to_string(), "0".to_string(), Pnt::ZeroPoint));
                }
            }
        }*/
        RftTable::new(range_cols, point_cols_inferred)
    }
}

impl ir_l1::Component for RftTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let col_count = self.range_cols.len() + self.point_cols.len() + 1;
        const FULL_TABLE_DIM : usize = 3;
        let col_dim = format!("{}in", (FULL_TABLE_DIM as f64) / (col_count as f64));
        let mut dims = vec![];
        for _ in 0..col_count {
            dims.push(col_dim.clone());
        }
        let mut ranges = vec!["\\( \\displaystyle x \\)".to_string()];
        let mut signs = vec!["\\( \\displaystyle y' \\)".to_string()];
        let mut rfs = vec!["\\( \\displaystyle y \\)".to_string()];
        for i in 0..self.point_cols.len() {
            if i == 0 {
                ranges.push("\\( \\displaystyle x < \\)".to_string());
            } else {
                ranges.push("\\( \\displaystyle < x < \\)".to_string());
            }
            signs.push(self.range_cols[i].sign_translated().to_string());
            rfs.push(self.range_cols[i].rf_translated().to_string());

            ranges.push(format!("\\( \\displaystyle {} \\)", crate::latexify::latexify_statements(&hwc_lang_equation::parse_statements(self.point_cols[i].0.clone()).unwrap())));
            signs.push(self.point_cols[i].1.sign_translated().to_string());
            rfs.push(self.point_cols[i].1.rf_translated().to_string());
        }
        ranges.push("\\( \\displaystyle < x \\)".to_string());
        signs.push(self.range_cols.last().unwrap().sign_translated().to_string());
        rfs.push(self.range_cols.last().unwrap().rf_translated().to_string());

        let entries = vec![ranges, signs, rfs];

        ir_l2_components.push(Box::new(ir_l2::Table::new(dims, entries, true)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

