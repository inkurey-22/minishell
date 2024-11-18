use std::env;
use std::io::{self, Write};
use std::process::Command;
use colored::Colorize;
mod builtins;

fn run_help()
{
    println!("curry-rust: a simple shell written in Rust");
    println!("Takes TCSH as a reference for environment commands");
    println!("Disclaimer: This is a toy project made for learning purposes");
    println!();
    println!("Builtins:");
    println!("  cd <dir>                Change the current directory to <dir>");
    println!("  env                     Print the environment variables");
    println!("  setenv <var> <value>    Set an environment variable");
    println!("  unsetenv <var>          Remove an environment variable");
    println!("  help                    Print this help message");
    println!("  Can also run any system command");
}

fn check_builtins(command: &str, args: &[&str], env_vars: &mut Vec<(String, String)>) -> bool
{
    match command {
        "cd" => {
            builtins::run_cd(args);
            true
        }
        "env" => {
            builtins::run_env(env_vars);
            true
        }
        "setenv" => {
            builtins::run_setenv(args, env_vars);
            true
        }
        "unsetenv" => {
            builtins::run_unsetenv(args, env_vars);
            true
        }
        "help" => {
            run_help();
            true
        }
        _ => false,
    }
}

fn execute_command(input: &str, env_vars: &mut Vec<(String, String)>)
{
    let input = input.trim();
    if input.is_empty() {
        return;
    }
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    if check_builtins(command, &args, env_vars) {
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

fn prompt()
{
    let current_dir = env::current_dir().unwrap();
    let home_dir = env::var("HOME").unwrap();
    let display_dir = current_dir.display().to_string();
    let display_dir =  if display_dir.starts_with(&home_dir) {
        display_dir.replacen(&home_dir, "~", 1)
    } else {
        display_dir
    };
    println!("{}", display_dir.blue());
    print!("{} > ", "curry-rust".green().bold());
    io::stdout().flush().unwrap();
}

fn run_shell(mut env_vars: Vec<(String, String)>)
{
    loop {
        prompt();
        let input = read_command();
        if input.trim() == "exit" {
            println!("exit");
            break;
        }
        execute_command(&input, &mut env_vars);
        println!();
    }
}

fn main()
{
    let env_vars: Vec<(String, String)> = env::vars().collect();
    run_shell(env_vars);
}
