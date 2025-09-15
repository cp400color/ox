use std::io::Read;
use std::path::Path;
use std::fs::File;

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

    println!("{:?}", oxfile) // do something here
}