use std::io::Read;
use std::path::Path;
use std::fs::File; // todo (for @fgclue): HEY you lazy. you have a stashed connit

#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    PackageManager{manager: String, reason: String},
    Recommend{package: String, reason: String},
    Os{os: String, reason: String},
    Env{env: String, reason: String}
}

#[allow(dead_code)]
#[derive(Debug)]
enum ASTItem {
    Command(Command),
    Negative(Command),
    Or(Vec<Command>)
}

pub fn recommend() {
    let path = Path::new(".ox/oxfile");
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't recommend anything {}", why),
        Ok(file) => file,
    };

    let mut oxfile = String::new();
    match file.read_to_string(&mut oxfile) {
        Err(why) => panic!("couldn't recommend anything {}", why),
        Ok(_) => (),
    };

    let mut ast: Vec<ASTItem> = vec!();

    for line in oxfile.split("\n") {
        if line == "" {
            continue;
        }
        
        let split: Vec<&str> = line.split(" ").collect();
        // todo: Find something that makes sense to do here. I am not letting 12AM me do this. Goodnight
    }

    println!("{:?}", ast);
}