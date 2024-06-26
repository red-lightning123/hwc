use crate::{ ir_l1, ir_l2 };

pub struct Table {
    dims : Vec<String>,
    rows : Vec<Vec<String>>,
    flip_h : bool
}

impl Table {
    pub fn new(dims : Vec<String>, rows : Vec<Vec<String>>, flip_h : bool) -> Table {
        Table {
            dims,
            rows,
            flip_h
        }
    }

    pub fn try_from_cbml_element(table_element : &hwc_lang_cbml::tags::format::Element) -> Result<Table, String> {
        let dims_element =
            match table_element.children().iter().rev().find(|child| child.name() == "dims") {
                Some(dims) => dims,
                None => return Err("ir_l1::Table::try_from_cbml_element - table with no dims element".to_string())
            };
        let dims_text = dims_element.text();
        let mut dims = vec![];
        for dim in dims_text.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>() {
            dims.push(dim.to_string());
        }

        let rows_element =
            match table_element.children().iter().rev().find(|child| child.name() == "rows") {
                Some(rows) => rows,
                None => return Err("ir_l1::Table::try_from_cbml_element - table with no rows element".to_string())
            };
        let rows_text = rows_element.text();
        let mut rows = vec![];
        for row in rows_text.replace("#p", "#").split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().chunks(dims.len()) {
            rows.push(row.to_vec().into_iter().map(|entry| ir_l2::latexify_table_math_text_entry(entry.to_string())).collect::<Result<Vec<_>, String>>()?);
        }
        Ok(Table::new(dims, rows, table_element.properties().get("flip_h").map(|t| if t == "false" { false } else if t == "true" { true } else { false /*shouldn't be ignored*/ }).unwrap_or(true)))
    }

    /*pub fn parse(string : String, column_count : usize) -> Table {
        let mut rows = vec![];
        for row in string.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().chunks(column_count) {
            rows.push(row.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }
        Table::new(rows)
    }*/
}

impl ir_l1::Component for Table {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        /*let mut entries = vec![];
        entries.push(vec!["s".to_string(), "v".to_string(), "t".to_string(), "מסלול".to_string()]);
        for row in self.rows.iter() {
            entries.push(row.to_vec());
        }

        entries = entries.into_iter().map(|entry| entry.iter().rev().map(|s| s.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();*/

        ir_l2_components.push(Box::new(ir_l2::Table::new(self.dims.clone(), self.rows.clone(), self.flip_h)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

