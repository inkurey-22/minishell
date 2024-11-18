use std::env;
use std::io::{self, Write};
use std::process::Command;
use colored::Colorize;

fn execute_command(input: &str, env_vars: &[(String, String)])
{
    let input = input.trim();
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    if command == "cd" {
        if let Some(dir) = args.get(0) {
            if let Err(e) = env::set_current_dir(dir) {
                eprintln!("cd: {}", e);
            }
        } else {
            eprintln!("cd: missing argument");
        }
        return;
    }
    let mut cmd = Command::new(command);
    cmd.args(&args);
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    match cmd.spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(_e) => {
            eprintln!("curry-rust: command not found: {}", command);
        }
    }
}

fn read_command() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn run_shell(env_vars: Vec<(String, String)>)
{
    loop {
        let current_dir = env::current_dir().unwrap();
        let home_dir = env::var("HOME").unwrap();
        let display_dir = current_dir.display().to_string();
        let display_dir =  if display_dir.starts_with(&home_dir) {
            display_dir.replacen(&home_dir, "~", 1)
        } else {
            display_dir
        };
        print!("{}\n{} > ", display_dir.blue().bold(), "curry-rust".green().bold());
        io::stdout().flush().unwrap();

        let input = read_command();
        if input.trim() == "exit" {
            println!("exit");
            break;
        }
        execute_command(&input, &env_vars);
        println!();
    }
}

fn main()
{
    let env_vars: Vec<(String, String)> = env::vars().collect();
    run_shell(env_vars);
}
