use std::collections::HashMap;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::LazyLock;

type CommandFn = fn(Option<&str>) -> Result<(), Err>;
type Err = Box<dyn std::error::Error>;

static BUILTINS: LazyLock<HashMap<&'static str, CommandFn>> = LazyLock::new(|| {
    HashMap::from([
        ("exit", shell_exit as CommandFn),
        ("echo", echo as CommandFn),
        ("type", type_ as CommandFn),
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
            None => println!("{}: command not found", command),
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

    println!("DEBUG PATH: {:?}", path);

    env::split_paths(&path)
        .map(|path| path.join(command))
        .find(|path| is_executable(path))
}

fn is_executable(file: &PathBuf) -> bool {
    match file.metadata() {
        Ok(meta) => meta.is_file() && (meta.permissions().mode() & 0111) != 0,
        Err(_) => false,
    }
}
