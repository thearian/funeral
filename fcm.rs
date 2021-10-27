use std::env;
use std::process::Command;
use std::str;

fn main() {
    let commands = get_env_args();
    let (executer, args) = declare_args(& commands);
    let result = run_code(executer, args);
    println!("Runned:\n{}", result);
}

fn get_env_args() -> Vec<String> {
    env::args().collect()
}

fn run_code(executer: String, args: Vec<String>) -> String {
    let str_args = args.iter()
        .map(|string| string.as_str());
    let res_buffer = Command::new(executer.as_str())
        .args(str_args)
        .output()
        .expect("Failed to execute process")
        .stdout;
    println!("Running {} {:?}", executer, args );
    return String::from(
        str::from_utf8(&res_buffer)
            .expect("Faild to translate str to buffer")
    )
}

fn declare_args(commands: &Vec<String>) -> (String, Vec<String>) {
    let executer: String;
    let mut args: Vec<String> = Vec::new();
    match commands[1].as_ref() {
        "gen" => {
            executer = String::from("py");
            args.push(String::from("./gen-map.py"));
            args.push( commands[2].to_owned() );
        },
        "run" => {
            executer = String::from("cargo");
            for cmd_arg in commands[2..].iter() {
                args.push( cmd_arg.to_string() );
            }
        }
        _ => {panic!("Command not found {:?}", commands)}
    };
    (executer, args)
}