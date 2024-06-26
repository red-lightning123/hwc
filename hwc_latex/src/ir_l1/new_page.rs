use crate::{ ir_l1, ir_l2 };

pub struct NewPage;

impl NewPage {
    pub fn new() -> NewPage {
        NewPage
    }
}

impl ir_l1::Component for NewPage {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::NewPage::new()) as Box<dyn ir_l2::Component>);
        components
    }
}
