mod exec;
mod parse;
use std::process;
use std::io::{self, Write};
use std::collections::HashMap;
#[allow(dead_code)]
fn pristrflush(s:&str){
    print!("{}",s);
    io::stdout().flush().unwrap();
}
fn wfinput() -> String{
    io::stdout().flush().unwrap();
    pristrflush("esh>> ");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(_) => {},
        Err(e) => {println!("Failed to perform very basic I/O operation on stdin: {}",e);process::exit(1)}
    };
    buffer
}
fn main() {
    println!("Hello! Welcome to esh. (i fucking suck)");
    // It is not recommended to use global variables (which i always use) so ill store the variables in main
    let mut envvars: HashMap<String,String> = HashMap::new();
    envvars.insert("wd".to_string(),"/".to_string());
    loop{
       let mut i = wfinput();
       i = i.replace("\n", "");
       parse::parseinp_exec(i,&mut envvars);
    }
}
