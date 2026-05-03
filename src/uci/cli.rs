use crate::uci;
use std::collections::HashMap;
use std::io::{self, Write};

trait CliCommand {
    fn run(&self, parameters: &[&str]);
}

struct InfoCommand;

impl CliCommand for InfoCommand {
    fn run(&self, _parameters: &[&str]) {
        write_line("Rudim v2.0 by znxftw");
    }
}

struct UciCommand;

impl CliCommand for UciCommand {
    fn run(&self, parameters: &[&str]) {
        uci::run(parameters);
    }
}

pub fn run() {
    let mut commands: HashMap<&str, Box<dyn CliCommand>> = HashMap::new();
    commands.insert("info", Box::new(InfoCommand));
    commands.insert("uci", Box::new(UciCommand));

    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            continue;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let command = parts[0];
        let parameters = &parts[1..];

        if command == "exit" {
            break;
        }

        if let Some(cli_command) = commands.get(command) {
            cli_command.run(parameters);
        } else {
            write_line(&format!("Unknown command {command}"));
        }

        let _ = io::stdout().flush();
    }
}

pub fn write_line(message: &str) {
    println!("{message}");
}
