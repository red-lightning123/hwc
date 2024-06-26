use crate::{ ir_l1, ir_l2 };

pub struct Subsubquestion {
    number : String
}

impl Subsubquestion {
    pub fn new(number : String) -> Subsubquestion {
        Subsubquestion {
            number
        }
    }

    fn translated_formatted(&self) -> String {
        let number_lowercase = self.number.to_lowercase();
        if number_lowercase == "i" || number_lowercase == "ii" || number_lowercase == "iii" || number_lowercase == "iv" || number_lowercase == "v" || number_lowercase == "vi" || number_lowercase == "vii" || number_lowercase == "viii" || number_lowercase == "ix" || number_lowercase == "x" {
            format!("({}", self.number)
        } else if let Ok(_) = self.number.parse::<i32>() {
            format!("{})", self.number)
        } else {
            println!("[WARNING] ir_l1::Subsubquestion::translated_formatted: unrecognized number \"{}\". translating to \"\" instead", self.number);
            String::new()
        }
    }
}

impl ir_l1::Component for Subsubquestion {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::InlineText::new(self.translated_formatted())) as Box<dyn ir_l2::Component>);
        components.push(Box::new(ir_l2::NewLine::new()) as Box<dyn ir_l2::Component>);
        components
    }
}
