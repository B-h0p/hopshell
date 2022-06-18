use crate::functions;
use std::fs::{self, metadata};

pub fn echo(msg : Vec<&str>) {
    let mut txt : String = String::from("");
    for x in msg {
        txt.push_str(x);
        txt.push_str(" ");
    }
    txt.remove(txt.len()-1);
    println!("{}", txt);
}

pub fn math_eval(expression : Vec<&str>) {
    let mut express_string : String = String::from("");
    for x in &expression {express_string.push_str(x)}
    let mut valid_expression : bool = true;
    
    { //CHECKS FOR A VALID EXPRESSION
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

pub fn list_dir() {
    let mut directory : String = functions::get_dir(); //dont really like the new dependency but whatever...
    directory.pop(); directory.pop(); //removes the tailing '>'
    let filenames = fs::read_dir(&directory).unwrap();
    for x in filenames {
        let file_lineage : String = x.unwrap().file_name().to_str().unwrap().to_string();
        if metadata(&file_lineage).unwrap().is_dir() {println!("    |-> <DIR>    {}", file_lineage)}
        else {println!("    |-> {}", file_lineage)};
    }
}