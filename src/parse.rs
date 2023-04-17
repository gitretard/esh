use crate::exec;
use std::collections::HashMap;
// Copied code from chat gpt
pub fn plsworkproperly(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();
    for c in s.chars() {
        if c == '\'' {
            in_quotes = !in_quotes;
        }
        if c == ' ' && !in_quotes {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        result.push(current.clone());
    }
    // idk how to remove the '' and convert it to &str because i am so fucking stupid my dad left me so ill iterate over the entire vector again again
    for n in 0..result.len() {
        result  /*woah i didnt knew this works*/  [n] /*lol*/=       /*man never thought this would work in any language*/result[n].replace/*woosh*/("'", "");
    }
    result
}
// shitty code at its finest!
// Implement relative path
pub fn parseinp_exec(s: String, envvars: &mut HashMap<String, String>) {
    let s: Vec<String> = plsworkproperly(s.as_str());
    if s.len() < 2{
        return;
    }
    match s[0].as_str() {
        "run" => {
            for mut i in 2..s.len() {
                if s[i] == "with" {
                    i += 1;
                    let mut args = String::new();
                    for i in i..s.len() {
                        if s[i].starts_with("$"){
                            let outp = exec::getoutp(s[i].as_str(),envvars);
                            args += outp.as_str();
                            continue;
                        }
                        args += &s[i];
                    }
                    exec::perform(s[1].to_string(), args, envvars);
                    return;
                }
            }
            exec::perform(s[1].to_string(), "".to_string(), envvars);
        }
        "set" => {
            if s.len() < 3 {
                println!("Invalid syntax! (length of Vec<String> is more than 3 and idk how to fucking detect errors so yeah)");
                return;
            }
            if s[2] != "to" {
                println!("What? change {} ??? {}", s[1], s[2]);
                return;
            }
            envvars.insert(s[1].to_string(), s[3].to_string());
        }
        "get" =>{
            let v = match envvars.get(&s[1]){
                None => {println!("{} is not set",s[1]);return;},
                Some(n) => n
            };
            println!("{v}")
        }
        n => {
            println!("Unknown: {}; I am not sure what you want me to do",n)
        }
    }
}
