use std::{io::{self, Write}, env, fs};
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

pub fn command_new_line(dir : String) -> String { //glorified get_string lol
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

pub fn generate_files_vec(to_lowercase : bool) -> Vec<String> { //creates a vector of the files which exist in the current dir
    let directory : String = get_dir();
    let mut file_vec : Vec<String> = Vec::from([]);         
    let filenames = fs::read_dir(&directory).unwrap();
    for x in filenames {
        let mut file : String = x.unwrap().file_name().to_str().unwrap().to_string();
        if to_lowercase {file = file.to_lowercase();}
        file_vec.push(file);}
    return file_vec;
}

//command-specific function/s
pub fn invert_gamma(ans : u32) -> (f32, u32) {
    let mut increment : u32 = 0;
    //s_g(seed) is the approximation we are trying to make
    //(temp_)ans is the actual answer to the equation
    //we are trying to get seed to a value so s_g(seed) is around ~98% accurate to ans
    //if our s_g(seed) is SMALLER than ans, we need to increase our seed (binaric pattern should work)
    //if our s_g(seed) is LARGER than ans, then we need to decrease our seed (...)
    
    let t_ans : f32 = ans as f32;
    let mut seed : f32 = 6.0; //note that this seed is by no means reflective of the distribution of factorials...
    //I ain't bothered to do that math
    let mut diviser : f32 = 6.0;
    while ((t_ans / stirling_gamma(seed) >= 0.99) && (t_ans / stirling_gamma(seed)) <= 1.01) == false {
        diviser = diviser / 2.0; //gives the program a binary search vibe.
        if stirling_gamma(seed) < t_ans {seed = seed + diviser;}
        else {seed = seed - diviser;}
        increment = increment + 1;
    }

    return (seed, increment);
}

fn stirling_gamma(xput : f32) -> f32 {
    const E : f32 = 2.7182;
    const TWO_PI : f32 = 6.2831;
    let part_a : f32 = xput.powf(xput)/ E.powf(xput);
    let part_b : f32  = (TWO_PI * xput).sqrt();
    return part_a * part_b;
}