use crate::{ ir_l1, ir_l2 };

pub struct Geo {
    parsed : hwc_lang_equation::format::GeoProof
}

impl Geo {
    fn new(parsed : hwc_lang_equation::format::GeoProof) -> Geo {
        Geo {
            parsed
        }
    }

    pub fn parse(geo_proof : String) -> Result<Geo, String> {
        Ok(Geo::new(hwc_lang_equation::parse_geo_proof(geo_proof)?))
    }
}

impl ir_l1::Component for Geo {
    fn to_ir_l2_components(&self, dict : &json::JsonValue) -> Vec<Box<dyn ir_l2::Component>> {
        let mut ir_l2_components = vec![];
        let mut step_entries = vec![];
        step_entries.push(vec!["טענה".to_string(), "נימוק".to_string()]);
        for step in self.parsed.steps() {
            let mut s = String::new();
            s += "\\begin{tabular}{c}\n";
            for item in step.multiline_statements().items() {
                s += &match item {
                    hwc_lang_equation::format::MultilineStatementsItem::Statements(statements) => {
                        let functional_text =
                            if let Some((name, args)) = crate::equation_helpers::statements_as_function(statements) {
                                if name == "qed" {
                                    Some(
                                        if args.is_empty() {
                                            "\\underline{מ. ש. ל.} ".to_string()
                                        } else {
                                            let mut translated_qed_ns = vec![];
                                            for arg in args {
                                                if let Some(qed_n) = crate::equation_helpers::statements_as_value(arg) {
                                                    let qed_n =
                                                        if qed_n == "A" {
                                                            "א"
                                                        } else if qed_n == "B" {
                                                            "ב"
                                                        } else if qed_n == "C" {
                                                            "ג"
                                                        } else if qed_n == "D" {
                                                            "ד"
                                                        } else if qed_n == "E" {
                                                            "ה"
                                                        } else if qed_n == "F" {
                                                            "ו"
                                                        } else if qed_n.parse::<usize>().is_ok() {
                                                            qed_n
                                                        } else {
                                                            panic!("unrecognized qed name: \"{}\"", qed_n)
                                                        };
                                                    translated_qed_ns.push(qed_n);
                                                } else {
                                                    panic!("qed name isn't a value")
                                                }
                                            }
                                            let mut s = "\\underline{מ. ש. ל. ".to_string();
                                            s += translated_qed_ns[0];
                                            for qed_n in &translated_qed_ns[1..] {
                                                s += " ";
                                                s += qed_n;
                                            }
                                            s += "} ";
                                            s
                                        }
                                    )
                                } else if name == "is" {
                                    if let [statements, class] = &args[..] {
                                        Some(format!("\\( \\displaystyle {} \\) \\\\\n", translate_is_statement(statements, class, dict)))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                        if let Some(text) = functional_text {
                            text
                        } else {
                            format!("\\( \\displaystyle {} \\) \\\\\n", crate::latexify::latexify_statements(statements))
                        }
                    }
                    hwc_lang_equation::format::MultilineStatementsItem::Newline(_) => "".to_string()
                };
            }
            s += "\\end{tabular}";
            let expl = step.explanation().expression();
            step_entries.push(vec![s, translate_explanation(expl, dict)]);
        }
        ir_l2_components.push(Box::new(ir_l2::Table::new(vec!["4in".to_string(), "2in".to_string()], step_entries, false)) as Box<dyn ir_l2::Component>);
        ir_l2_components
    }
}

fn translate_explanation(explanation : &hwc_lang_equation::format::Expression, dict : &json::JsonValue) -> String {
    let v = vec![];
    let (explanation_name, explanation_args) =
        if let Some(explanation_name) = crate::equation_helpers::expression_as_value(explanation) {
            (explanation_name, &v)
        } else if let Some((explanation_name, explanation_args)) = crate::equation_helpers::expression_as_function(explanation) {
            (explanation_name, explanation_args)
        } else {
            panic!("explanation is in invalid format")
        };
    let explanation_entry = &dict["expls"][explanation_name];
    if explanation_entry.is_null() {
        panic!("entry \"{}\" is missing in explanations dictionary", explanation_name)
    } else {
        if explanation_args.is_empty() {
            if let Some(template) = explanation_entry.as_str() {
                return template.to_string();
            }
        }
        let template = &explanation_entry[explanation_args.len().to_string()];
        if template.is_null() {
            panic!("entry \"{}\" found in explanations dictionary, but has no variant for {} arguments", explanation_name, explanation_args.len())
        } else {
            let mut explanation_translated = template.as_str().unwrap();
            let mut _temp = String::new();
            for (i, arg) in explanation_args.iter().enumerate() {
                _temp = explanation_translated.replace(&format!("${}", i + 1), &crate::latexify::latexify_statements(arg));
                explanation_translated = &_temp;
            }
            explanation_translated.to_string()
        }
    }
}

fn translate_is_statement(statements : &hwc_lang_equation::format::Statements, class : &hwc_lang_equation::format::Statements, dict : &json::JsonValue) -> String {
    let v = vec![];
    let (class_name, class_args) =
        if let Some(class_name) = crate::equation_helpers::statements_as_value(class) {
            (class_name, &v)
        } else if let Some((class_name, class_args)) = crate::equation_helpers::statements_as_function(class) {
            (class_name, class_args)
        } else {
            panic!("is-class is in invalid format")
        };
    let class_entry = &dict["is"][class_name];
    if class_entry.is_null() {
        panic!("entry \"{}\" is missing in statements dictionary", class_name)
    } else {
        if class_args.is_empty() {
            if let Some(template) = class_entry.as_str() {
                return template.replace("$0", &crate::latexify::latexify_statements(statements));
            }
        }
        let template = &class_entry[class_args.len().to_string()];
        if template.is_null() {
            panic!("entry \"{}\" found in is-statements dictionary, but has no variant for {} arguments", class_name, class_args.len())
        } else {
            let mut statement_translated = template.as_str().unwrap();
            let mut temp = statement_translated.replace("$0", &crate::latexify::latexify_statements(statements));
            statement_translated = &temp;
            for (i, arg) in class_args.iter().enumerate() {
                temp = statement_translated.replace(&format!("${}", i + 1), &crate::latexify::latexify_statements(arg));
                statement_translated = &temp;
            }
            statement_translated.to_string()
        }
    }
}
