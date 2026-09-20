use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::sync::LazyLock;

type CommandFn = fn(Option<&str>) -> Result<(), Err>;
type Err = Box<dyn std::error::Error>;

static COMMANDS: LazyLock<HashMap<&'static str, CommandFn>> = LazyLock::new(|| {
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

        match COMMANDS.get(command) {
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
        Some(command) => match COMMANDS.get(command) {
            None => {
                println!("{}: not found", command);
                Ok(())
            }
            Some(_) => {
                println!("{} is a shell builtin", command);
                Ok(())
            }
        },
    }
}
