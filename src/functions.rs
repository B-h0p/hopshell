use std::{io::{self, Write}, env};
use clearscreen;

pub fn new_bash(msg : String) {
    clearscreen::clear().unwrap(); //shoutout to watchexec on GitHub
    println!("{}", msg);}

pub fn get_dir() -> String {
    let dir_raw : String = env::current_dir().unwrap().to_str().unwrap().to_string(); //cursed!
    let dir_split : Vec<&str> = dir_raw.split("").collect();
    let mut dir : String = "".to_string();
    for x in dir_split {dir.push_str(x);}
    return dir;
}

pub fn command_new_line(dir : String) -> String {
    print!("{}", dir);
    io::stdout().flush().unwrap();
    let mut output : String = String::new();
    let reader = io::stdin();
    reader.read_line(&mut output)
        .expect("Failed to read line");
    while (output.ends_with("\n")) || (output.ends_with("\r")) || (output.ends_with(" ")) {output.pop();}
    return output;
}

pub fn get_string(remove_opening_spaces : bool) -> String {
    let mut output : String = String::new();
    let reader = io::stdin();
    reader.read_line(&mut output)
        .expect("Failed to read line");
    
        while (output.ends_with("\n")) || (output.ends_with("\r")) || (output.ends_with(" ")) {
            output.pop();}      
        if remove_opening_spaces {
            while (output.starts_with("\n")) || (output.starts_with("\r")) || (output.starts_with(" ")) {
                output.remove(0);
        }}
        return output
    }