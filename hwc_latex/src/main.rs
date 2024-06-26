mod ir_l1;
mod ir_l2;
mod latexify;
mod equation_helpers;
use std::fs;
use std::env;
use std::process;
use std::io::Write;

fn get_main_minifile_content(file : hwc_lang_cbml::preproc::resolved_file::ResolvedFile) -> Result<String, String> {
    if let Some(minifile) = file.minifiles().get("main") {
        Ok(minifile.to_string())
    } else {
        Err("no minifile named \"main\" in file".to_string())
    }
}

fn main() {
    /*loop {
        let mut data = String::new();
        loop {
            let mut line = String::new();
            use std::io::Read;
            std::io::stdin().read_line(&mut line);
            if line.trim() == "STOP" {
                break;
            } else if line.trim() == "STOPT" {
                data = data.trim().to_string();
                break;
            }/* else {
                data += &line;
                data = data[..data.bytes().len()-1].to_string();
                break;
            }*/
            data += &line;
        }
        println!("highlighted:");
        highlight_cbml_file_tags(&data);
        println!("parsing result:");
        use crate::cbml::tags::parse::ParseTokens;
        let mut tokens = &*cbml::tags::lex_file(&data);
        let parsed = cbml::tags::parse::ElementArray::parse_tokens_mut(&mut tokens);
        match parsed {
            Ok(_) => { println!("success:"); cbml::tags::parse::debug_highlight_tokens(tokens); },
            Err(e) => {
                println!("{}", e);
            }
        }
    }*/
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("no input file supplied");
            return
        }
    };
    let raw_file = fs::read_to_string(file_path).unwrap();
    let preprocessed_file = match hwc_lang_cbml::preproc::parse_file(raw_file) {
        Ok(preprocessed_file) => preprocessed_file,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let resolved_preprocessed_file = match hwc_lang_cbml::preproc::resolve_file(preprocessed_file) {
        Ok(resolved_file) => resolved_file,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let main_preprocessed_minifile = match get_main_minifile_content(resolved_preprocessed_file) {
        Ok(main_minifile) => main_minifile,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let cbml_tag_file = match hwc_lang_cbml::tags::parse_file(main_preprocessed_minifile) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    
    let dict = json::parse(include_str!("dict.json")).unwrap();

    let document_ir_l1 = match ir_l1::Document::try_from_cbml_tag_file(cbml_tag_file, dict) {
        Ok(document) => document,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let document_ir_l2 = document_ir_l1.into_ir_l2();
    let document_latex = document_ir_l2.write_latex();


    match env::set_current_dir("doc") {
        Ok(_) => { }
        Err(e) => {
            println!("set working directory operation failed: {}", e);
            return
        }
    }

    write!(fs::File::create("doc.latex").unwrap(), "{}", document_latex).unwrap();

    let latex_prompt_output = process::Command::new("xelatex")
        .args(&["doc.latex"])
        .output()
        .unwrap().stdout;
    let latex_prompt_output = String::from_utf8(latex_prompt_output).unwrap();

    for line in latex_prompt_output.lines() {
        if line.contains("! ") {
            println!("{}", line);
        }
    }
}
