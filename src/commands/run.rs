use std::process::Command;
use clap::parser::ValuesRef;
use anstyle::Style;

fn get_script_path(script: Option<&String>, subscript: Option<&String>) -> String {
    let root: &'static str = ".ox/";
    let folder: &String = script.unwrap();
    let file: String = match subscript {
        Some(script) => format!("/{}", script),
        None => "".to_string()
    };

    return format!("{}{}{}", root, folder, file)
}

pub fn run(script: Option<&String>, subscript: Option<&String>, extras: Option<ValuesRef<'_, std::string::String>>) -> () {
    const BOLD_STYLE: Style = Style::new().bold();

    let path: String = get_script_path(script, subscript);
    println!("Running {BOLD_STYLE}{path}{BOLD_STYLE:#}...");

    let args = match extras {
        Some(_) => extras.unwrap().collect(),
        None => vec!()
    };

    if cfg!(target_os = "windows") {
        Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
    } else {
        Command::new("bash")
            .arg(path)
            .args(args)
            .status()
            .expect("failed to execute process");
    }
}