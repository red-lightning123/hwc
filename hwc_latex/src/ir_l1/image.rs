use crate::{ ir_l1, ir_l2 };

pub struct Image {
    path : String,
    width : String
}

impl Image {
    pub fn new(path : String, width : String) -> Image {
        Image {
            path,
            width
        }
    }
}

impl ir_l1::Component for Image {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::Image::new(self.path.clone(), self.width.clone())) as Box<dyn ir_l2::Component>);
        components
    }
}
