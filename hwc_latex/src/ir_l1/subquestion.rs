use crate::{ ir_l1, ir_l2 };

pub struct Subquestion {
    number : String
}

impl Subquestion {
    pub fn new(number : String) -> Subquestion {
        Subquestion {
            number
        }
    }

    fn number_translated(&self) -> &str {
        if self.number.len() == 1 {
            let ch = self.number.chars().next().unwrap(); // unwrap cannot fail since self.number.len() == 1
            match ch {
                'A' => "א",
                'B' => "ב",
                'C' => "ג",
                'D' => "ד",
                'E' => "ה",
                'F' => "ו",
                'G' => "ז",
                'H' => "ח",
                'I' => "ט",
                'J' => "י",
                'K' => "יא",
                'L' => "יב",
                'M' => "יג",
                'N' => "יד",
                'O' => "טו",
                'P' => "טז",
                'Q' => "יז",
                'R' => "יח",
                'S' => "יט",
                'T' => "כ",
                'U' => "כא",
                'V' => "כב",
                'W' => "כג",
                'X' => "כד",
                'Y' => "כה",
                'Z' => "כו",
                _ => {
                    println!("[WARNING] ir_l1::Subquestion::number_translated: unrecognized number character \'{}\'. translating to \"\" instead", self.number);
                    ""
                }
            }
        } else {
            println!("[WARNING] ir_l1::Subquestion::number_translated: number string \"{}\" is not a single character. translating to \"\" instead", self.number);
            ""
        }
    }
}

impl ir_l1::Component for Subquestion {
    fn to_ir_l2_components(&self, _dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut components = vec![];
        components.push(Box::new(ir_l2::InlineText::new(format!("{}.", self.number_translated()))) as Box<dyn ir_l2::Component>);
        components.push(Box::new(ir_l2::NewLine::new()) as Box<dyn ir_l2::Component>);
        components
    }
}
