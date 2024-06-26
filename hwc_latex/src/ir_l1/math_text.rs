use crate::{ ir_l1, ir_l2 };

pub struct MathText {
    parsed : hwc_lang_dollar::format::File
}

impl MathText {
    fn new(parsed : hwc_lang_dollar::format::File) -> MathText {
        MathText {
            parsed
        }
    }

    pub fn parse(math_text : String) -> Result<MathText, String> {
        Ok(MathText::new(hwc_lang_dollar::parse_file(math_text)?))
    }
}

impl ir_l1::Component for MathText {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        for component in self.parsed.components() {
            ir_l2_components.append(&mut match_math_text_component_to_ir_l2_components(component).map_err(|e| { println!("{}", e); e }).unwrap());
        }
        ir_l2_components
    }
}

fn match_math_text_component_to_ir_l2_components(component : &hwc_lang_dollar::format::Component) -> Result<Vec<Box<dyn ir_l2::Component>>, String> {
    Ok(match component {
        hwc_lang_dollar::format::Component::InlineText(text) => {
            vec![Box::new(ir_l2::InlineText::new(text.to_string())) as Box<dyn ir_l2::Component>]
        }
        hwc_lang_dollar::format::Component::InlineMath(math) => {
            vec![Box::new(ir_l2::InlineMath::parse(math.to_string())?) as Box<dyn ir_l2::Component>]
        }
        hwc_lang_dollar::format::Component::MultilineMath(math) => {
            let multiline_statements = &hwc_lang_equation::parse_multiline_statements(math.to_string())?;
            let mut items = vec![];
            for item in multiline_statements.items() {
                items.push(match item {
                    hwc_lang_equation::format::MultilineStatementsItem::Statements(statements) => {
                        Box::new(ir_l2::InlineMath::new((*statements).clone())) as Box<dyn ir_l2::Component>
                    }
                    hwc_lang_equation::format::MultilineStatementsItem::Newline(_) => {
                        Box::new(ir_l2::NewLine::new()) as Box<dyn ir_l2::Component>
                    }
                });
            }
            items
        }
        hwc_lang_dollar::format::Component::Newline => {
            vec![Box::new(ir_l2::NewLine::new()) as Box<dyn ir_l2::Component>]
        }
    })
}
