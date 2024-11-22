use std::env;

pub fn run_cd(args: &[&str])
{
    if let Some(dir) = args.get(0) {
        if let Err(e) = env::set_current_dir(dir) {
            eprintln!("cd: {}", e);
        }
    } else {
        if let Err(e) = env::set_current_dir(env::home_dir().unwrap()) {
            eprintln!("cd: {}", e);
        }
    }
}

pub fn run_env(env_vars: &[(String, String)])
{
    for (key, value) in env_vars {
        println!("{}={}", key, value);
    }
}

pub fn run_setenv(args: &[&str], env_vars: &mut Vec<(String, String)>)
{
    if args.len() != 2 {
        eprintln!("setenv: missing argument");
        return;
    }
    let key = args[0];
    let value = args[1];
    if let Some((_, v)) = env_vars.iter_mut().find(|(k, _)| k == key) {
        *v = value.to_string();
    } else {
        env_vars.push((key.to_string(), value.to_string()));
    }
}

pub fn run_unsetenv(args: &[&str], env_vars: &mut Vec<(String, String)>)
{
    if args.len() != 1 {
        eprintln!("unsetenv: missing argument");
        return;
    }
    let key = args[0];
    env_vars.retain(|(k, _)| k != key);
}
