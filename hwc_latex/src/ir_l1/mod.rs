use std::collections::HashMap;
use crate::ir_l2;

pub trait Component {
    fn to_ir_l2_components(&self, dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>>;
}

pub struct Document {
    title : String,
    author : String,
    date : String,
    components : Vec<Box<dyn Component>>,
    dict : json::JsonValue
}

impl Document {
    pub fn new(title : String, author : String, date : String, components : Vec<Box<dyn Component>>, dict : json::JsonValue) -> Document {
        Document {
            title,
            author,
            date,
            components,
            dict
        }
    }

    pub fn into_ir_l2(self) -> ir_l2::Document {
        let mut ir_l2_components = vec![];

        for component in self.components {
            ir_l2_components.append(&mut component.to_ir_l2_components(&self.dict));
        }

        ir_l2::Document::new(
            self.title,
            self.author,
            self.date,
            ir_l2_components
        )
    }

    pub fn try_from_cbml_tag_file(file : hwc_lang_cbml::tags::format::File, dict : json::JsonValue) -> Result<Document, String> {
        let mut names_to_elements = HashMap::new();

        for element in file.root_element().children() {
            names_to_elements.insert(element.name().as_str(), element);
        }

        let title = names_to_elements["title"].text().to_string();
        let author = names_to_elements["author"].text().to_string();
        let date = names_to_elements["date"].text().to_string();
        let mut components = vec![];

        for component_tag in names_to_elements["content"].children() {
            let component_name = component_tag.name();
            if component_name == "intro" {
                components.push(Box::new(Intro::new()) as Box<dyn Component>);
            } else if component_name == "np" {
                components.push(Box::new(NewPage::new()) as Box<dyn Component>);
            } else if component_name == "sec" {
                let name = component_tag.properties().get("name").ok_or("ir_l1::Document::try_from_cbml_tag_file - sec is missing name property".to_string())?.to_string();
                components.push(Box::new(Section::new(name)) as Box<dyn Component>);
            } else if component_name == "ssec" {
                let name = component_tag.properties().get("name").ok_or("ir_l1::Document::try_from_cbml_tag_file - ssec is missing name property".to_string())?.to_string();
                components.push(Box::new(Subsection::new(name)) as Box<dyn Component>);
            } else if component_name == "sssec" {
                let name = component_tag.properties().get("name").ok_or("ir_l1::Document::try_from_cbml_tag_file - sssec is missing property".to_string())?.to_string();
                components.push(Box::new(Subsubsection::new(name)) as Box<dyn Component>);
            } else if component_name == "q" {
                let question_number = component_tag.properties().get("n").ok_or("ir_l1::Document::try_from_cbml_tag_file - q is missing n property".to_string())?.to_string();
                components.push(Box::new(Question::new(question_number)) as Box<dyn Component>);
            } else if component_name == "sq" {
                let subquestion_number = component_tag.properties().get("n").ok_or("ir_l1::Document::try_from_cbml_tag_file - sq is missing n property".to_string())?.to_string();
                components.push(Box::new(Subquestion::new(subquestion_number)) as Box<dyn Component>);
            } else if component_name == "ssq" {
                let subsubquestion_number = component_tag.properties().get("n").ok_or("ir_l1::Document::try_from_cbml_tag_file - ssq is missing n property".to_string())?.to_string();
                components.push(Box::new(Subsubquestion::new(subsubquestion_number)) as Box<dyn Component>);
            } else if component_name == "mt" {
                let math_text = component_tag.text().to_string();
                components.push(Box::new(MathText::parse(math_text)?) as Box<dyn Component>);
            } else if component_name == "svtn" {
                let svtn_content = component_tag.text().to_string();
                components.push(Box::new(SvtnTable::parse(svtn_content)) as Box<dyn Component>);
            } else if component_name == "table" {
                components.push(Box::new(Table::try_from_cbml_element(component_tag)?) as Box<dyn Component>);
            } else if component_name == "img" {
                let path = component_tag.properties().get("path").ok_or("ir_l1::Document::try_from_cbml_tag_file - img is missing path property".to_string())?.to_string();
                let width = component_tag.properties().get("width").unwrap_or(&"\\linewidth".to_string()).to_string();
                components.push(Box::new(Image::new(path, width)) as Box<dyn Component>);
            } else if component_name == "geo" {
                let geo_string = component_tag.text().to_string();
                components.push(Box::new(Geo::parse(geo_string)?) as Box<dyn Component>);
            } else if component_name == "pn" {
                let pn_content = component_tag.text().to_string();
                components.push(Box::new(PnTable::parse(pn_content)) as Box<dyn Component>);
            } else if component_name == "pnc" {
                components.push(Box::new(PncTable::try_from_cbml_element(component_tag)?) as Box<dyn Component>);
            } else if component_name == "rf" {
                let rf_content = component_tag.text().to_string();
                components.push(Box::new(RfTable::parse(rf_content)) as Box<dyn Component>);
            } else if component_name == "rft" {
                let rft_content = component_tag.text().to_string();
                components.push(Box::new(RftTable::parse(rft_content)) as Box<dyn Component>);
            } else if component_name == "un" {
                let un_content = component_tag.text().to_string();
                components.push(Box::new(UnTable::parse(un_content)) as Box<dyn Component>);
            } else if component_name == "c" {
            } else {
                return Err(format!("unrecognized tag: {}", component_name))
            }
        }

        Ok(Document::new(title, author, date, components, dict))
    }
}

mod intro;
pub use intro::Intro;

mod new_page;
pub use new_page::NewPage;

mod section;
pub use section::Section;

mod subsection;
pub use subsection::Subsection;

mod subsubsection;
pub use subsubsection::Subsubsection;

mod question;
pub use question::Question;

mod subquestion;
pub use subquestion::Subquestion;

mod subsubquestion;
pub use subsubquestion::Subsubquestion;

mod math_text;
pub use math_text::MathText;

mod svtn_table;
pub use svtn_table::SvtnTable;

mod pn_table;
pub use pn_table::PnTable;

mod pnc_table;
pub use pnc_table::PncTable;

mod rf_table;
pub use rf_table::RfTable;

mod rft_table;
pub use rft_table::RftTable;

mod un_table;
pub use un_table::UnTable;

mod table;
pub use table::Table;

mod image;
pub use image::Image;

mod geo;
pub use geo::Geo;
