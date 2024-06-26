use crate::{ ir_l1, ir_l2 };

use ir_l1::Table;

pub struct PncTable {
    table : Table
}

impl PncTable {
    fn new(table : Table) -> PncTable {
        PncTable {
            table
        }
    }

    pub fn try_from_cbml_element(table_element : &hwc_lang_cbml::tags::format::Element) -> Result<PncTable, String> {
        let dims_element =
            match table_element.children().iter().rev().find(|child| child.name() == "dims") {
                Some(dims) => dims,
                None => return Err("ir_l1::PncTable::try_from_cbml_element - table with no dims element".to_string())
            };
        let dims_text = dims_element.text();
        let mut dims = vec![];
        for dim in dims_text.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>() {
            dims.push(dim.to_string());
        }

        let rows_element =
            match table_element.children().iter().rev().find(|child| child.name() == "rows") {
                Some(rows) => rows,
                None => return Err("ir_l1::PncTable::try_from_cbml_element - table with no rows element".to_string())
            };
        let rows_text = rows_element.text();
        let mut rows = vec![];
        let rows_text_replaced = rows_text.replace("#p", "#");
        let rows_text_replaced = rows_text_replaced.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let mut rows_text_it = rows_text_replaced.chunks(dims.len());
        if let Some(row) = rows_text_it.next() {
            rows.push(row.to_vec().into_iter().map(|entry| ir_l2::latexify_table_math_text_entry(entry.to_string())).collect::<Result<Vec<_>, String>>()?);
        }
        for row_text in rows_text_it {
            let mut row = vec![];
            let mut row_it = row_text.into_iter();
            if let Some(first_entry) = row_it.next() {
                row.push(ir_l2::latexify_table_math_text_entry(first_entry.to_string())?);
                //rows.push(row.to_vec().into_iter().map(|entry| ir_l2::latexify_table_math_text_entry(entry.to_string())).collect::<Result<Vec<_>, String>>()?);
            }
            for entry in row_it {
                if *entry == "+" {
                    row.push(ir_l2::latexify_table_math_text_entry("$ {} + {} $".to_string())?);
                } else if *entry == "-" {
                    row.push(ir_l2::latexify_table_math_text_entry("$ {} - {} $".to_string())?);
                } else if *entry == "u" {
                    row.push("\\BigCross".to_string());
                } else {
                    return Err("ir_l1::PncTable::try_from_cbml_element - invalid entry".to_string())
                }
            }
            rows.push(row);
        }
        Ok(PncTable::new(Table::new(dims, rows, table_element.properties().get("flip_h").map(|t| if t == "false" { false } else if t == "true" { true } else { false /*shouldn't be ignored*/ }).unwrap_or(true))))
    }

    /*pub fn parse(string : String, column_count : usize) -> Table {
        let mut rows = vec![];
        for row in string.split("#").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().chunks(column_count) {
            rows.push(row.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }
        Table::new(rows)
    }*/
}

impl ir_l1::Component for PncTable {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        self.table.to_ir_l2_components(_dict)
    }
}

