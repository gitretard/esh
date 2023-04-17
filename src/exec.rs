use crate::parse;
use std::{collections::HashMap, process};
pub fn perform(cmd: String, args: String, envvars: &mut HashMap<String, String>) {
    // copypaste due to me not knowing how to work with the borrow checker and i suck
    match cmd.as_str() {
        "ls" => {
            let mut args = args;
            if args == "" {
                args = match envvars.get("wd") {
                    None => {
                        println!("GODDAMNIT I HATE OPTIONS<>");
                        return;
                    }
                    Some(n) => n.clone(),
                };
            }
            let mut proc = match process::Command::new(cmd).arg(args).spawn() {
                Err(e) => {
                    println!("esh: failed to ls: {}", e);
                    return;
                }
                Ok(n) => n,
            };
            proc.wait().unwrap();
        }
        _ => {
            let mut proc = match process::Command::new(&cmd).arg(args).spawn() {
                Ok(e) => e,
                Err(n) => {
                    println!("esh: failed to perform {}: {}", cmd, n);
                    return;
                }
            };
            proc.wait().unwrap();
        }
    }
}
//TODO merge into perform or parseinp_exec (sorry for copypaste)
// yeah i mean 2 FUCKING FUNCTIONS to do nearly the same thing isnt that good
pub fn getoutp(cmd: &str,envvars: &mut HashMap<String, String>) -> String {
    let mut t = String::new();
    for ch in cmd.chars() {
        if ch == '$' {
            continue;
        }
        t.push(ch)
    }
    let n: Vec<String> = parse::plsworkproperly(t.as_str());
    if n.len() < 2{
        return "".to_string();
    }
    match n[0].as_str() {
        "run" => {
            let mut args: String = String::new();
            for mut i in 0..n.len() {
                if n[i] == "with" {
                    i += 1;
                    for i in i..n.len() {
                        args += n[i].as_str()
                    }
                }
            }
            let proc = match process::Command::new(n[1].as_str()).arg(args).output() {
                Ok(c) => c,
                Err(n) => {
                    println!("Failed to get output of {cmd} {n}");
                    return "".to_string();
                }
            };
            let stdout = String::from_utf8_lossy(&proc.stdout).to_string();
            stdout.replace("\n","")
        }
        "get" => {
            let n = match envvars.get(&n[1]){
                Some(n) => n,
                None => {println!("{} not set!",n[1]);return "".to_owned();}
            };
            n.to_string()
        }
        _ => return "".to_owned(),
    }
}
