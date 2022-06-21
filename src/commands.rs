use crate::functions;
use std::{fs::{self, File, metadata},env};
use meval;

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
    if expression.len() > 0 { //expression needed
        let mut express_string : String = String::from("");
        for x in &expression {express_string.push_str(x)}
        let mut valid_expression : bool = true;
    
        { //CHECKS FOR A VALID EXPRESSION
            let expression_chars : Vec<char> = express_string.chars().collect();
            let mut contains_number : bool = false;
            let mut lbrace_count : u32 = 0;
            let mut rbrace_count : u32 = 0;
            for x in 0..expression_chars.len() {
                if !"0987654321+-/*()^".contains(expression_chars[x]) {valid_expression = false; break;} //only good symbols allowed
                if "0987654321".contains(expression_chars[x]) {contains_number = true;} //a number is AT LEAST needed
                if expression_chars[x] == '(' {
                    lbrace_count = lbrace_count + 1;
                    if x != 0 {
                        if !"+-*/^(".contains(expression_chars[x-1]) {valid_expression = false; break;}
                    }}//a number can't immeadietaly preceed a open bracket - it has to be explicit
                if expression_chars[x] == ')' {
                    rbrace_count = rbrace_count + 1;
                    if x != expression_chars.len()-1 {
                        if !"+-*/^)".contains(expression_chars[x+1]) {valid_expression = false; break;} //see ln 35
                    }}
                if rbrace_count > lbrace_count {valid_expression = false; break;} //at all points '('.len >= ')'.len
            }
            if !contains_number {valid_expression = false;}
            if lbrace_count != rbrace_count {valid_expression = false;} //bracket count must be equal

            if !("0987654321(-".contains(expression_chars[0])) || ("+-*/^".contains(expression_chars[expression_chars.len()-1])) {
                valid_expression = false;} //must begin with a (+|-)number/ bracket, and end with a digit or bracket
            for x in 0..expression_chars.len()-1 {
                if (expression_chars[x] == '(') && (expression_chars[x+1] == ')') {valid_expression = false; break;}
                //empty brackets not allowed
                if "+-*/^".contains(expression_chars[x]) && !("0987654321(-".contains(expression_chars[x+1])) {
                    valid_expression = false; break;} //math operator needs to follow something reasonable
            }
        } //expression is now (presumably) valid
        
        if !valid_expression {println!("'{}' is not recognised as a valid expression. Try again.", express_string);}  
        else {
            let ans : f64 = meval::eval_str(&express_string).unwrap(); //THANK YOU REKKA!!
            println!("{}",ans);   
        }
    }
    else {println!("math statements must include an expression. Try again");}
}

pub fn list_dir() {
    let directory : String = functions::get_dir();
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
        let mut file_string : String = String::from("./"); //make cleaner TODO
        for x in filename {
            file_string.push_str(x);
            file_string.push_str(" ");}
        file_string.pop(); file_string = file_string.to_lowercase();
        let mut file_string_cleaned : String = file_string.clone();
        file_string_cleaned.remove(0); file_string_cleaned.remove(0); //removes the './' - kinda poor

        let file_vec : Vec<String> = functions::generate_files_vec(true);
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
            else {println!("you found the fortnite easter egg! ...thats all");} //to prevent panics lol
        }
        else {println!("'{}' already exists in this directory. Try again.", file_string);}
    }
    else {println!("new items requires a name. Try again.");}
}

pub fn delete_item(item : Vec<&str>) {
    if item.len() != 0 {
        let mut item_name : String = String::from("");
        for x in item {
            item_name.push_str(x);
            item_name.push_str(" ");}
        item_name.pop();

        let file_vec : Vec<String> = functions::generate_files_vec(true);
        if file_vec.contains(&item_name.to_lowercase()) {
            let mut file_to_delete : String = String::from("./");
            file_to_delete.push_str(&item_name);
            let mut user_confirmation : String = String::from("");
            while user_confirmation != "y".to_string() && user_confirmation != "n".to_string() {
                println!("delete {}? (y/N)", item_name);
                user_confirmation = functions::get_string(true).to_lowercase();
            }
            if user_confirmation == "y".to_string() {
                if metadata(&file_to_delete).unwrap().is_dir() {
                    fs::remove_dir(file_to_delete).expect("Err: Failed to delete directory");} //TO-FIX
                else {fs::remove_file(file_to_delete).expect("Err: Failed to delete file")} //TO-FIX
                println!("{} deleted.", item_name)
            }
        }
        else {println!("{} does not exist in this directory. Try again", item_name);}
       
    }
    else {println!("you need to specify an item to delete");}
}