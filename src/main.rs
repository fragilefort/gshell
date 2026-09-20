use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::sync::LazyLock;

type CommandFn = fn(Option<&str>) -> Result<(), Box<dyn std::error::Error>>;

static COMMANDS: LazyLock<HashMap<&'static str, CommandFn>> = LazyLock::new(|| {
    HashMap::from([
        ("exit", shell_exit as CommandFn),
        ("echo", echo as CommandFn),
    ])
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let (command, remainder) = buffer.trim().split_once(' ').unwrap_or((buffer.trim(), ""));

        match COMMANDS.get(command) {
            Some(fun) => call(*fun, Some(remainder))?,
            None => println!("{}: command not found", command),
        }
    }
}

fn call(f: CommandFn, args: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    f(args)
}

fn shell_exit(_args: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(0);
}

fn echo(_args: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match _args {
        None => Ok(()),
        Some(text) => {
            println!("{}", text);
            Ok(())
        }
    }
}
