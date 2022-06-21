mod functions;
mod commands;
const VERSION : &str = "authored by Bhop, 2022 :3 [Version 0.4.3]";

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
            _other => valid_command = false, //non-existent command
        }
    return valid_command;
}