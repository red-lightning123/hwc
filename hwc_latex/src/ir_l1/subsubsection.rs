use crate::{ ir_l1, ir_l2 };

pub struct Subsubsection {
    name : String
}

impl Subsubsection {
    pub fn new(name : String) -> Subsubsection {
        Subsubsection {
            name
        }
    }
}

impl ir_l1::Component for Subsubsection {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::Subsubsection::new(self.name.clone())) as Box<dyn ir_l2::Component>);
        components
    }
}
