use crate::{ ir_l1, ir_l2 };

pub struct Subsection {
    name : String
}

impl Subsection {
    pub fn new(name : String) -> Subsection {
        Subsection {
            name
        }
    }
}

impl ir_l1::Component for Subsection {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::Subsection::new(self.name.clone())) as Box<dyn ir_l2::Component>);
        components
    }
}
