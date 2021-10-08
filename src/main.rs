use std::fs;
use surd::{Surd};
use r256::{Styles};

fn main() {
    let mut app = Surd::new("axis", "Modern and minimal tool for searching a text in folder.", "Modeminal", "0.1.0");
    app.add_flag("path", "Set the current path.", vec!["p".to_string()]);
    app.set_handler(main_handler);

    r256::init();
    app.start();
}

fn main_handler(found_flags: &Vec<surd::Flag>, _: Vec<String>) {
    let mut program_path = &String::from(".");

    for flag in found_flags {
        if flag.name == "path" && flag.value != "" {
            program_path = &flag.value
        }
    }
    
    find_text(program_path);
}

fn find_text(program_path: &String) {
    match fs::read_dir(program_path) {
        Ok(rd) => {
            for path in rd {
                match path {
                    Ok(file) => {
                        println!("{:?}", file.file_name())
                    },
                    Err(err) => {
                        let err_vec: Vec<Styles> = vec![Styles::Bold, Styles::FgColor256(9)];
                        r256::println(&err_vec, &err.to_string());
                    }
                }
            }
        },

        Err(err) => {
            let err_vec: Vec<Styles> = vec![Styles::Bold, Styles::FgColor256(9)];
            r256::println(&err_vec, &err.to_string());
        }
    }
}