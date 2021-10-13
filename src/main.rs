use std::fs;
use surd::{Surd};
use r256::{Styles};

fn main() {
    let mut app = Surd::new("axis", "Modern and minimal tool for searching a text in folder.", "Modeminal", "0.1.0");
    app.add_flag("path", "Set the current path.", vec!["p".to_string()]);
    app.add_flag("wait", "Wait input for the next result.", vec!["w".to_string()]);
    app.set_handler(main_handler);

    r256::init();
    app.start();
}

fn main_handler(found_flags: &Vec<surd::Flag>, program_args: Vec<String>) {    
    if program_args.len() == 0 {
        return
    }
    
    let mut program_path = &String::from(".");
    let mut wait_for_another = false;

    for flag in found_flags {
        if flag.name == "path" && flag.value != "" {
            program_path = &flag.value;
        } else if flag.name == "wait" {
            wait_for_another = true;
        }
    }

    handle_path(program_path, program_args, wait_for_another);
}

// Iterate over files and directories in path.
fn handle_path(program_path: &String, program_args: Vec<String>, wait_for_another: bool) {
    match fs::read_dir(program_path) {
        Ok(rd) => {
            for path in rd {
                match path {
                    Ok(file) => wrap_file(file, program_args.clone(), wait_for_another),
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

// Wrap file
fn wrap_file(file: std::fs::DirEntry, program_args: Vec<String>, wait_for_another: bool) {
    // Get file metadata for check if it is a directory.
    match file.metadata() {
        Ok(metadata) => {
            let file_path = file.path().display().to_string();

            if metadata.is_dir() {
                // Recursive directory handling.
                handle_path(&file_path, program_args, wait_for_another)
            } else {
                // Read file data
                match fs::read_to_string(&file_path) {
                    Ok(file_data) => {
                        let printable_str = find_text(file_data, program_args.join(" "));

                        if printable_str.len() != 0 {
                            println!("{} [{}]\n{}{}\n", 
                              r256::generate_string(&vec![Styles::Bold, Styles::FgColor256(10)], &file_path),
                              r256::generate_string(&vec![Styles::FgColor256(15)], &metadata.len().to_string()),
                              "-".repeat(file_path.len()),
                              printable_str
                            );

                            // Wait an input if --walk is used.
                            if wait_for_another {
                                match clear_input("> ") {
                                    Ok(input) => {
                                        if input == "q" {
                                            std::process::exit(0);
                                        }
                                    },
                                    Err(err) => {
                                        let err_vec: Vec<Styles> = vec![Styles::Bold, Styles::FgColor256(9)];
                                        r256::println(&err_vec, &err.to_string());
                                    }
                                }
                            }
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

// Find text from data.
fn find_text(data: String, find: String) -> String {
    let mut formatted_text = String::new();

    for (index, line) in data.lines().enumerate() {
        // Check if line contains the text.
        if line.contains(&find) {
            // Replace the text with background-colored version of the text.
            let colored_found_text = r256::generate_string(&vec![Styles::BgColor256(4), Styles::FgColor256(15)], &find);
            let got_str = line.replace(&find, &colored_found_text);

            if line == got_str {
                continue
            }

            formatted_text = format!("{}\n{}:{}", formatted_text, r256::generate_string(&vec![Styles::Bold, Styles::FgColor256(11)], &(index + 1).to_string()), got_str)
        }
    }

    formatted_text
}

// Simple function for take clear input easily. 
fn clear_input(input: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin,stdout,Write};

    print!("{}", input);
    let _ = stdout().flush();

    let mut input_buffer = String::new();

    match stdin().read_line(&mut input_buffer) {
        Ok(_) => {
            if let Some('\n') = input_buffer.chars().next_back() {
                input_buffer.pop();
            }

            if let Some('\r') = input_buffer.chars().next_back() {
                input_buffer.pop();
            }

            Ok(input_buffer)
        },
        Err(err) => Err(err)
    }
}