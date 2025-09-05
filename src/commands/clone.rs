use std::process::Command;
use clap::parser::ValuesRef;
use anstyle::Style;

pub fn clone(repository: Option<&String>, threads: Option<&i32>, extras: Option<ValuesRef<'_, std::string::String>>) {
    const BOLD_STYLE: Style = Style::new().bold();
    let repository: &String = repository.unwrap();
    let threads = threads.unwrap();
    let args = match extras {
        Some(_) => extras.unwrap().collect(),
        None => vec!()
    };

    println!("Cloning {BOLD_STYLE}{repository}{BOLD_STYLE:#} with {BOLD_STYLE}{threads}{BOLD_STYLE:#} thread{}...", if threads == &(1 as i32) {""} else {"s"});

    Command::new("git")
        .arg("clone")
        .arg(repository)
        .arg("--recurse-submodules")
        .arg(format!("-j{threads}"))
        .args(args)
        .output()
        .expect("failed to execute process");
}