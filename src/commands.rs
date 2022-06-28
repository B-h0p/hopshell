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
            else {println!("ERR - item couldn't be created.");}
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

pub fn user_help(command_wargs : Vec<&str>) {
    if command_wargs.len() != 0 {
        match command_wargs[0] {
            "kill" => println!("KILL terminates the Hopshell console"),
            "cls" | "clear" => println!("CLS (clear) will clear the console's user inputs and system outputs."),
            "echo" | "print" => println!("ECHO (print) returns the users input-text to the console."),
            "math" | "calc" | "eval" => {
                println!("\nMATH (calc|eval) is a command which determines basic arithmetic expressions");
                println!("OPERATIONAL KEY:");
                println!("  '+' - Addition");
                println!("  '-' - Subtraction");
                println!("  '*' - Multiplication");
                println!("  '/' - Division");
                println!("  '^' - Exponentiation\n");
                println!("MATH will handle brackets ('()'), but numbers cannot be multiplied immediately");
                println!("by a fraction");
            },
            "ls" | "dir" | "sdir" => {
                println!("\nLS (dir|sdir) will print the contents of the current directory");
                println!("to the console. Note that directories will be distinguished from files.");
                println!("\nNOTE: directories critical to the OS will NOT be displayed")
            },
            "cd" | "cdir" => {
                println!("\nCD (cdir) will change the terminals directory to the users specification");
                println!("cd will operate with the following syntax:");
                println!("\n    - 'cd [DIR]'");
                println!("\nNOTE: using '..' for the [DIR] argument will return a directory lower to hopshell");
            },
            "newf" | "makf" | "makef" => {
                println!("\nNEWF (makf|makef) will create a new file according to the users specifications");
                println!("newf will operate with the following syntax:");
                println!("\n    - 'newf [FILE]'"); //NOTE: I may add a hidden argument soon with '-h' :)
                println!("\nNOTE: created files are recommended to have an assigned name and filetype");
            },
            "newd" | "makd" | "maked" => {
                println!("\nNEWD (makd|maked) will create a new directory according to the users specifications");
                println!("newd will operate with the following syntax:");
                println!("\n    - 'newd [DIR]'"); //NOTE: I may add a hidden argument soon with '-h' :)
            },
            "del" | "rmv" => {
                println!("\nDEL (rmv) will delete a certain directory or file that is specified by the user");
                println!("del will operate with the following syntax:");
                println!("\n    - 'del [ITEM]'");
                println!("\nNOTE: items can only be deleted with further user confirmation to avoid accidents");
                println!("(directories with contents in them will cause Hopshell to panic if they interact with del");
                println!("  fixing this is currently a work in progress)");
            },
            "help" => println!("help is a command that helps the user. Use 'help help' to see help about help."),
            _other => println!("'{}' is not a valid command. Try again.", command_wargs[0])
        }   //might use the Levenshtein function that I worked on earlier here ^
    }
    else {
        println!("\nHopshell available expressions:");
        println!("  kill: terminates Hopshell session");
        println!("  cls|clear: clears the console display.");
        println!("  echo|print: prints String to console");
        println!("  math|calc|eval: calculates simple expression");
        println!("  ls|dir|sdir: prints the current directory to console");
        println!("  cd|cdir: changes the directory to the users specification");
        println!("  newf|makf|makef: creates a new file named by the user");
        println!("  newd|makd|maked: creates a new DIR named by the user");
        println!("  del|rmv: deletes a specified DIR or file");
        println!("\nuse 'help [command]' to see more information for a given prompt.");
    }
}