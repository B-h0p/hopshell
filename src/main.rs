mod functions;
mod commands;
mod help;
const VERSION : &str = "authored by Bhop, 2022 :3 [Version 1.3.3]";

fn main() {
    functions::new_bash(VERSION.to_string());
    loop {
        let mut dir : String = functions::get_dir(); dir.push_str("> "); //for the aesthetic
        let instruction : String = functions::command_new_line(dir);
        if instruction != "".to_string() {
            let command : Vec<&str> = instruction.split(" ").collect::<Vec<&str>>(); //splitting by string allows us to add parameters
            if command[0].to_lowercase() == "kill".to_string() {break} //stupid edge case v1
            if !match_command(&command) {
                println!("'{}' is not a recognised command. Refer to hopshell's 'help' command", command[0]);
                println!("for more information.")
            }
            if !["cls".to_string(), "clear".to_string()].contains(&command[0].to_lowercase()) {println!("");} //stupid edge case v2
        }
    }
}

fn match_command(arg : &Vec<&str>) -> bool {
    let user_command : String = arg[0].to_string().to_lowercase();
    let mut expression = arg.clone();
    expression.remove(0); //arguments for certain (upcoming commands) - trust me, this wasnt dumb!
    let mut valid_command : bool = true; 
        match user_command.as_str() {
            "cls" | "clear" => functions::new_bash(VERSION.to_string()), //clears the screen
            "echo" | "print" => commands::echo(expression), //prints to command line
            "math" | "calc" | "eval" => commands::math_eval(expression), //evaluates math expressions
            "ls" | "dir" | "sdir" => commands::list_dir(), //lists directory contents
            "cd" | "cdir" => commands::change_dir(expression), //changes directory
            "newf" | "makf" | "makef" => commands::new_item(expression, "f"), //creates file
            "newd" | "makd" | "maked" => commands::new_item(expression, "d"), //creates directory
            "del" | "rmv" => commands::delete_item(expression), //deletes item
            "help" => help::user_help(expression), //user docs
            "ifct" => { //returns the inverted factorial value (e.g. 720 -> 6)
                if expression.len() != 0 {
                    let gamma_check : bool = expression[expression.len()-1].to_string().to_lowercase() == "-g".to_string();
                    if gamma_check {expression.remove(expression.len()-1);} //-g is removed
                    let mut numinp : String = String::from("");
                    for x in expression {numinp.push_str(x);}
                    commands::invert_factorial(numinp, gamma_check); //using a string instead of Vec<str> for code reusability
                }
                else {println!("A digit is needed. Try again.");}
            },
            "find" |"srch"| "quer" => {
                let case_insensitive : bool = expression[expression.len()-1].to_string().to_lowercase() == "-c".to_string();
                if case_insensitive {expression.remove(expression.len()-1);} //-g is removed
                commands::find_str(expression, case_insensitive)
            },
            _other => valid_command = false, //non-existent command
        }   //might use Levenshtein here ^
    return valid_command;
}