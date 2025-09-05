fn get_script_path(script: Option<&String>, subscript: Option<&String>) -> String {
    let root = ".ox/";
    let folder = script.unwrap();
    let file = match subscript {
        Some(script) => format!("/{}", script),
        None => "".to_string()
    };

    return format!("{}{}{}", root, folder, file)
}

pub fn run(script: Option<&String>, subscript: Option<&String>) {
    let path = get_script_path(script, subscript);
}