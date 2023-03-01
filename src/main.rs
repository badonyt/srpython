use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;
use regex::Regex;
use std::collections::HashMap;
use evalexpr::eval;
use evalexpr::Value;
#[derive(Debug)]
enum Value_var {
    Integer(i64),
    Float(f64),
    String(String),
}


fn remove_spaces_outside_quotes(input: &str) -> String {
    let mut output = String::new();
    let mut in_quotes = false;
    for c in input.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        }
        if c == ' ' && !in_quotes {
            continue;
        }
        output.push(c);
    }
    output
}
fn extract_bracket_contents(s: &str) -> Option<&str> {
    if let (Some(start), Some(end)) = (s.find('('), s.find(')')) {
        if start < end {
            return Some(&s[start + 1..end]);
        }
    }
    None
}
fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.split(b'\n') {
        let line = line.unwrap();
        lines.push(String::from_utf8_lossy(&line).to_string());
    }

    lines
}
fn is_python_loop(statement: &str) -> bool {
    let loop_keywords = ["while", "for", "async for"];
    let normalized_statement = statement.trim_start();

    for keyword in loop_keywords.iter() {
        if normalized_statement.starts_with(keyword) {
            return true;
        }
    }

    false
}

fn replace_all_occurrences(s: &str, target: &str, new: &str) -> Box<str> {
    let result = s.replace(target, new);
    result.into_boxed_str()
}
fn has_no_repeated_characters(s: &str, ch: char) -> bool {
    let mut set = HashSet::new();

    for c in s.chars() {
        if c == ch {
            if set.contains(&ch) {
                return false;
            }
            set.insert(ch);
        }
    }

    true
}
fn is_python_augmented_assignment(line: &str) -> bool {
    let augmented_assignment_regex = Regex::new(r"^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\+=\s*\d+\s*$").unwrap();
    augmented_assignment_regex.is_match(line)
}
fn is_python_var(line: &str) -> bool {
    let variable_regex = Regex::new(r"^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*.*").unwrap();
    variable_regex.is_match(line)
}
fn is_python_if(line: &str) -> bool{
    if line.len() < 3{return false}
    let first_char_vec: Vec<char> = line.trim().chars().collect();
    let goofy_value: Vec<char> = line.trim().chars().collect();
    if first_char_vec[0] == 'i' && first_char_vec[1] == 'f' && goofy_value[goofy_value.len() - 1] == '{'{
        return true
    }
    false
}
fn help_message(){
    println!("GENERIC --HELP MESSAGE");
}
fn code(lines: Vec<String>, variables: &mut HashMap<String, Value_var>){
    let mut i = 0;
    let mut ignore_lines: HashMap<i32,i32> = HashMap::new();
    for line in lines.clone(){
        //println!("{}", line);
       println!("{:?}",ignore_lines);
        let mut yes = false;
        for (&key, &value) in ignore_lines.iter() {
            if ((i+1) > key && i < value) && (i+1) != value {
                yes = true;
                break;
            }
        }
        if yes == true{continue;} 
       // println!("not nice: {}", line);
        if line.trim().split("(").next() == Some("print"){
            let extract_bracket_contents: Option<&str> = extract_bracket_contents(&line);
            if extract_bracket_contents.is_none(){
               println!("Error forgot brackets on line {}",i+1);
                std::process::exit(1);
            }
            let extract_bracket_contents = extract_bracket_contents.unwrap();
            //let mut divided: _ = extract_bracket_contents.expect("lol").split("+").collect::<B>();
            println!("{}",extract_bracket_contents);
            let mut divided: Vec<&str> =  extract_bracket_contents.split("+").collect();

            println!("{:?}",divided);
        }else if is_python_augmented_assignment(&line){
            let mut parts: Vec<&str> = line.trim().split("+").collect();
            parts[0] = parts[0].trim_end();
            println!("D:{:?}", parts);
            let parts: String = parts[1].trim().to_string().replace("=", "");
            println!("D:{:?}", parts);
           //im not sure my guys
        }else if is_python_var(&line) == true{
            let parts: Vec<&str> = line.trim().split("=").collect();
           // println!("{}", parts[0].replace(" ", ""));
            if line.starts_with('#') || line.starts_with('}'){continue;}
            if remove_spaces_outside_quotes(&parts[1]).contains('"') || remove_spaces_outside_quotes(&parts[1]).contains('\'') {
                let damn = String::from(parts[0].replace(" ", ""));
                variables.insert(damn, Value_var::String(replace_all_occurrences(&remove_spaces_outside_quotes(&parts[1]), "\"","").to_string() ));
            }else if remove_spaces_outside_quotes(&parts[1]).contains('"') == false || remove_spaces_outside_quotes(&parts[1]).contains('\'') == false{
                let damn = String::from(parts[0].replace(" ", ""));
               // println!("{:?}", parts[1].replace(" ", ""));

                //let parts: Vec<&str> = input.split(' ').collect();


                let parsed = String::from(parts[1].replace(" ", "")).parse::<f64>();
                match parsed {
                    Ok(number) => {
                        if number.fract() == 0.0 {
                            let integer = number as i64;
                            variables.insert(damn, Value_var::Integer(integer));
                        } else {
                            variables.insert(damn, Value_var::Float(number));
                        }
                    },
                    Err(_) => {
                        let parsed = String::from(parts[1].replace(" ", "")).parse::<i64>().unwrap();
                        variables.insert(damn, Value_var::Integer(parsed));
                    }
                }
                
                
            } 
            
            

        }else if is_python_if(&line){
            let extract_bracket_contents = extract_bracket_contents(&line);
            if extract_bracket_contents == None{
               println!("Error forgot brackets on line {}",i+1);
                std::process::exit(1);
            }

            let first_delimiter = "&&";
            let second_delimiter = "||";
            let mut modified_expression:String = "w".to_string();
            for substr in extract_bracket_contents.unwrap().split(second_delimiter) {
                for item in substr.split(first_delimiter) {
                   // println!("{}", item);
                    modified_expression = item
                                    .split_whitespace()
                                        .map(|token| match variables.get(token) {
                                                    Some(Value_var::Integer(i)) => Value::Int(*i).to_string(),
                                     Some(Value_var::Float(f)) => Value::Float(*f).to_string(),
                                     Some(Value_var::String(s)) => Value::String(s.clone()).to_string(),
                                           //Some(Value_var::Boolean(b)) => Value::Bool(*b).to_string(),
                                       None => token.to_string(),
                                            })
                              .collect::<Vec<_>>()
                                              .join(" ");
                    //println!("{}",modified_expression);
                    
                }
            }
            if eval(&modified_expression) == Ok(Value::from(false)){
                let mut y = 0;
                let mut deep = 0;
                for is in &lines[(i as usize)+1..]{
                   // println!("line  {}",is);
                    if is.trim().ends_with("{"){
                        deep +=1;
                    }
                    if is.trim().starts_with("}") && deep == 0{ignore_lines.insert((i+1).try_into().unwrap(),(i+y+2).try_into().unwrap());break;}else if is.trim().starts_with("}"){deep -=1;}
                    y +=1;
                }
            }

        }else if is_python_loop(&line){
            let goofy_value: &str = extract_bracket_contents(&line).unwrap();
            let goofy_value: Vec<&str> = goofy_value.split(",").collect();
            let mut y = 0;
            let mut deep = 0;
            for is in &lines[(i as usize)+1..]{
               // println!("line  {}",is);
                if is.trim().ends_with("{"){
                    deep +=1;
                }
                if is.trim().starts_with("}") && deep == 0{ignore_lines.insert((i+1).try_into().unwrap(),(i+y+2).try_into().unwrap());break;}else if is.trim().starts_with("}"){deep -=1;}
                y +=1;
            }   
            if goofy_value.len() == 1{
                let mut fo = goofy_value[0].to_string().parse::<i32>().unwrap();
                    println!("{}", fo);
                for i in 0..fo{
                    if let Some((&key, &value)) = ignore_lines.iter().last() {
                        println!("The most recent key-value pair is {}: {}", key, value);
                        let latest_key = key;
                        let latest_value = value;
                        code((&lines[(key as usize)..(value as usize)]).to_vec(),  variables);
                        println!("Latest key: {}", latest_key);
                        println!("Latest value: {}", latest_value);
                    } else {
                        println!("The map is empty");
                    }
                    
                }
                
            }
            
            
            
            
            println!("{:?}", goofy_value);
        }
     /*   let x = variables.get("x").unwrap();

        match x {
            Value_var::Integer(i) => { /* handle integer Value_var */ },
            Value_var::Float(f) => { /* handle float Value_var */ },
            Value_var::String(s) => { /* handle string Value_var */ },
        }*/
        i+=1;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut variables: HashMap<String, Value_var> = HashMap::new();
    let file_path = &args[1];
    let lines = read_file_lines(file_path);
    if args.len() < 2 {
        help_message();
        std::process::exit(1);
    }
    if args[1] == "--help"{
        help_message();
        std::process::exit(1);
    }
    code(lines, &mut variables);
    
    println!("{:?}",variables);
    
}