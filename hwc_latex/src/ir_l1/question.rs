use crate::{ ir_l1, ir_l2 };

pub struct Question {
    number : String
}

impl Question {
    pub fn new(number : String) -> Question {
        Question {
            number
        }
    }
}

impl ir_l1::Component for Question {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::InlineText::new(format!("{}.", self.number))) as Box<dyn ir_l2::Component>);
        components.push(Box::new(ir_l2::NewLine::new()) as Box<dyn ir_l2::Component>);
        components
    }
}
