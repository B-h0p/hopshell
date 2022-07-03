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
                println!("  (Trying to delete an OS critical DIR will crash Hopshell - be careful!)");
            },
            "ifct" => {
                println!("\nIFCT will return the inverted factorial of a given integer");
                println!("for example: ");
                println!("\n 720 -> 6");
                println!("\nNOTE: ifct may use the '-g' arguement, which employs the gamma approximation");
                println!("  for numbers without a clean factorial inversion");
            },
            "find" => {
                println!("\nFIND will return specified Strings of text that are found in a given file");
                println!("find will operate with the following syntax:");
                println!("\n    - 'find [FILENAME] |MSG| [STRING]'");
                println!("\nNOTE: find may use the '-c' argument, which will ignore case-sensitivity.");
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
        println!("  ifct: returns the inverted factorial of an integer");
        println!("  find: returns extracts of specified text from a given file");
        println!("\nuse 'help [command]' to see more information for a given prompt.");
    }
}