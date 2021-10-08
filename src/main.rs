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

fn main_handler(found_flags: &Vec<surd::Flag>, program_args: Vec<String>) {    
    if program_args.len() == 0 {
        return
    }
    
    let mut program_path = &String::from(".");

    for flag in found_flags {
        if flag.name == "path" && flag.value != "" {
            program_path = &flag.value
        }
    }
    handle_path(program_path, program_args);
}

fn handle_path(program_path: &String, program_args: Vec<String>) {
    match fs::read_dir(program_path) {
        Ok(rd) => {
            for path in rd {
                match path {
                    Ok(file) => wrap_file(file, program_args.clone()),
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

fn wrap_file(file: std::fs::DirEntry, program_args: Vec<String>) {
    match file.metadata() {
        Ok(metadata) => {
            let file_path = file.path().display().to_string();

            if metadata.is_dir() {
                handle_path(&file_path, program_args)
            } else {
                match fs::read_to_string(&file_path) {
                    Ok(file_data) => {
                        let printable_str = find_text(file_data, program_args.join(" "));

                        if printable_str.len() != 0 {
                            println!("{} [{}]\n{}{}\n", 
                              r256::generate_string(&vec![Styles::Bold, Styles::FgColor256(10)], &file_path),
                              r256::generate_string(&vec![Styles::FgColor256(15)], &metadata.len().to_string()),
                              "-".repeat(file_path.len()),
                              printable_str
                            )
                        }
                    },
                    Err(_) => return
                }
            }
        },
        Err(err) => {
            let err_vec: Vec<Styles> = vec![Styles::Bold, Styles::FgColor256(9)];
            r256::println(&err_vec, &err.to_string());
        }
    }
}

fn find_text(data: String, find: String) -> String {
    let mut formatted_text = String::new();

    for (index, line) in data.lines().enumerate() {
        if line.contains(&find) {
            let colored_found_text = r256::generate_string(&vec![Styles::BgColor256(4), Styles::FgColor256(15)], &find);
            let got_str = line.replace(&find, &colored_found_text);

            if line == got_str {
                continue
            }

            formatted_text = format!("{}\n{}:{}", formatted_text, r256::generate_string(&vec![Styles::Bold, Styles::FgColor256(11)], &index.to_string()), got_str)
        }
    }

    formatted_text
}