mod functions;
mod commands;
const VERSION : &str = "authored by Bhop, 2022 :3 [Version 0.4.2]";

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
    expression.remove(0); 
        match user_command.as_str() {
            "cls" | "clear" => { //clears the interface
                functions::new_bash(VERSION.to_string());
                return true}, 
            "echo" | "print" => { //prints input to screen
                commands::echo(expression);
                return true}  
            "math" | "calc" | "eval" => { //basic arithmetic
                commands::math_eval(expression);
                return true}
            "ls" | "dir" | "sdir" => { //lists directory contents
                commands::list_dir();
                return true}
            "cd" | "cdir" => { //change directory
                commands::change_dir(expression);
                return true}
            "newf" | "makf" | "makef" => { //make new FILE
                commands::new_item(expression, "f");
                return true}
            "newd" | "makd" | "maked" => { //make new DIR
                commands::new_item(expression, "d");
                return true}
            "del" | "rmv" => { //delete any item
                commands::delete_item(expression);
                return true} //apparently I cant abstract these return trues, not without an overhaul at least
            _other => return false,
        }
}