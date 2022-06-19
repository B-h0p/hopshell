use crate::functions;
use std::fs::{self, File, metadata};
use std::env;

pub fn echo(msg : Vec<&str>) {
    if msg.len() > 0 {
        let mut txt : String = String::from("");
        for x in msg {
            txt.push_str(x);
            txt.push_str(" ");
        }
        txt.remove(txt.len()-1);
        println!("{}", txt);
    }
}

pub fn math_eval(expression : Vec<&str>) {
    if expression.len() > 0 {
        let mut express_string : String = String::from("");
        for x in &expression {express_string.push_str(x)}
        let mut valid_expression : bool = true;
        
        { //CHECKS FOR A VALID EXPRESSION - TO BE REFACTORED
            let expression_chars : Vec<char> = express_string.chars().collect();
            let mut contains_number : bool = false;
            for x in 0..expression_chars.len() {
                if !"0987654321+-/*()^".contains(expression_chars[x]) {valid_expression = false; break;}
                if "0987654321".contains(expression_chars[x]) {contains_number = true;}
            }
            if !contains_number {valid_expression = false;}

            let mut lbrace_count : u32 = 0;
            let mut rbrace_count : u32 = 0;
            for x in 0..expression_chars.len() {
                if expression_chars[x] == '(' {
                    lbrace_count = lbrace_count + 1;
                    if x != 0 {
                        if !"+-*/^(".contains(expression_chars[x-1]) {valid_expression = false; break;}
                    }}
                if expression_chars[x] == ')' {
                    rbrace_count = rbrace_count + 1;
                    if x != expression_chars.len()-1 {
                        if !"+-*/^)".contains(expression_chars[x+1]) {valid_expression = false; break;}
                    }}
                if rbrace_count > lbrace_count {valid_expression = false; break;}
                //so we don't deal with empty brackets in the future
                if x != expression_chars.len() - 1 {
                    if (expression_chars[x] == '(') && (expression_chars[x+1] == ')') {valid_expression = false; break;}
                }
            }
            if lbrace_count != rbrace_count {valid_expression = false;}

            if !("0987654321(-".contains(expression_chars[0])) || ("+-*/^".contains(expression_chars[expression_chars.len()-1])) {
                valid_expression = false;}
            for x in 0..expression_chars.len()-1 {
                if "+-*/^".contains(expression_chars[x]) && !("0987654321(-".contains(expression_chars[x+1])) {
                    valid_expression = false; break;}
            }
        } //expression is now valid
        if !valid_expression {println!("'{}' is not recognised as a valid expression. Try again.", express_string);}  
        else {
            let ans : f64 = meval::eval_str(&express_string).unwrap(); //THANK YOU REKKA!!
            println!("{}",ans);   
        }
    }
    else {println!("math statements must include an expression. Try again");}
}

pub fn list_dir() {
    let mut directory : String = functions::get_dir(); //dont really like the new dependency but whatever...
    directory.pop(); directory.pop(); //removes the tailing '>'
    let mut directory_temp : String = directory.clone(); directory_temp.pop();
        let filenames = fs::read_dir(&directory).unwrap();
        for x in filenames {
            let file_lineage : String = x.unwrap().file_name().to_str().unwrap().to_string();
            let valid_check = metadata(&file_lineage);
            let is_ok : bool;
            match valid_check {
                Ok(_) => is_ok = true,
                Err(_) => is_ok = false
            }
            if is_ok { //only files which can be tampered with *safely* are shown
                if metadata(&file_lineage).unwrap().is_dir() {println!("    |-> <DIR>    {}", file_lineage)}
                else {println!("    |-> {}", file_lineage)};
            }
        }
}

pub fn change_dir(dir : Vec<&str>) {
    if dir.len() != 0 {
        let old_dir : String = functions::get_dir();
        let mut dir_string : String = String::from("./");
        for x in dir {
            dir_string.push_str(x);
            dir_string.push_str(" ");
        }
        dir_string.pop();
        let is_ok = env::set_current_dir(&dir_string);
        match is_ok {
            Err(_) => (), //ehh????
            Ok(_) => ()
        } //i dont get rust...
        let new_dir : String = functions::get_dir();
        if old_dir == new_dir {println!("'{}' is not a valid directory. Try again", dir_string);}
    }
    else {println!("A directory is needed as an argument. Try again");}
}

pub fn new_item(filename : Vec<&str>, itype : &str) { //TODO -REFACTOR THIS TRASH
    if filename.len() != 0 {
        let mut file_string : String = String::from("./");
        for x in filename {
            file_string.push_str(x);
            file_string.push_str(" ");}
        file_string.pop(); file_string = file_string.to_lowercase();

        let mut directory : String = functions::get_dir(); directory.pop(); directory.pop();
        let mut file_vec : Vec<String> = Vec::from([]);         
        let filenames = fs::read_dir(&directory).unwrap();
        for x in filenames {
            let file : String = x.unwrap().file_name().to_str().unwrap().to_string().to_lowercase();
            file_vec.push(file);} //used to check if the file we are making doesnt exist

        let mut file_string_cleaned : String = file_string.clone();
        file_string_cleaned.remove(0); file_string_cleaned.remove(0);
        if !(file_vec.contains(&file_string_cleaned)) {
            if itype == "f" { //lots of semantics for this lol
                if !(&file_string_cleaned.contains(".")) {println!("WARNING: file has no assigned type");}
                if (file_string_cleaned.chars().into_iter().collect::<Vec<char>>()[0] == '.') && 
                   (file_string_cleaned.matches(".").count() == 1) {
                    println!("WARNING: file has no assigned name");}
                if file_string.chars().into_iter().collect::<Vec<char>>()[file_string.len()-1] == '.' {
                    println!("WARNING: files ending in '.' are ignored");}
                let is_ok = File::create(&file_string);
                match is_ok {
                    Ok(_) =>  println!("{} created in current directory.", file_string),
                    Err(_) => println!("new file couldn't be created.")}
            }
            else if itype == "d" { //as long as the directory is titled its cool
                let is_ok = fs::create_dir(&file_string);
                match is_ok {
                    Ok(_) => println!("created {} as a new directory.", file_string),
                    Err(_) => println!("new directory couldn't be created")}
            }
            else {println!("specified item format not found");}
        }
        else {println!("'{}' already exists in this directory. Try again.", file_string);}
    }
    else {println!("new items requires a name. Try again.");}
}
