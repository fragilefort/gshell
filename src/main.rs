use std::collections::HashMap;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::LazyLock;

type CommandFn = fn(Option<&str>) -> Result<(), Err>;
type Err = Box<dyn std::error::Error>;

static BUILTINS: LazyLock<HashMap<&'static str, CommandFn>> = LazyLock::new(|| {
    HashMap::from([
        ("exit", shell_exit as CommandFn),
        ("echo", echo as CommandFn),
        ("type", type_ as CommandFn),
        ("pwd", pwd as CommandFn),
    ])
});

fn main() -> Result<(), Err> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let (command, remainder) = buffer.trim().split_once(' ').unwrap_or((buffer.trim(), ""));

        match BUILTINS.get(command) {
            Some(fun) => call(*fun, Some(remainder))?,
            None => match find_exec(command) {
                Some(_) => execute_program(&command, remainder),
                None => println!("{}: command not found", command),
            },
        }
    }
}

fn call(f: CommandFn, args: Option<&str>) -> Result<(), Err> {
    f(args)
}

fn shell_exit(_args: Option<&str>) -> Result<(), Err> {
    std::process::exit(0);
}

fn echo(_args: Option<&str>) -> Result<(), Err> {
    match _args {
        None => Ok(()),
        Some(text) => {
            println!("{}", text);
            Ok(())
        }
    }
}

fn type_(_args: Option<&str>) -> Result<(), Err> {
    match _args {
        None => Ok(()),
        Some(command) => match BUILTINS.get(command) {
            Some(_) => {
                println!("{} is a shell builtin", command);
                Ok(())
            }
            None => match find_exec(command) {
                Some(path) => {
                    println!("{} is {}", command, path.display());
                    Ok(())
                }
                None => {
                    println!("{}: not found", command);
                    Ok(())
                }
            },
        },
    }
}

fn find_exec(command: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    env::split_paths(&path)
        .map(|path| path.join(command))
        .find(|path| is_executable(path))
}

fn is_executable(file: &Path) -> bool {
    file.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn execute_program(command: &str, args: &str) {
    let output = Command::new(command)
        .args(args.split_whitespace())
        .output()
        .expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
}

fn pwd(_args: Option<&str>) -> Result<(), Err> {
    println!("{}", env::current_dir()?.display());
    Ok(())
}
