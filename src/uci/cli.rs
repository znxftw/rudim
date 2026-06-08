use crate::datagen;
use crate::uci;
use std::collections::HashMap;
use std::io::{self, Write};

trait CliCommand {
    fn run(&self, parameters: &[&str]);
}

struct InfoCommand;

impl CliCommand for InfoCommand {
    fn run(&self, _parameters: &[&str]) {
        write_line(&format!("Rudim v{} by znxftw", env!("CARGO_PKG_VERSION")));
    }
}

struct UciCommand;

impl CliCommand for UciCommand {
    fn run(&self, parameters: &[&str]) {
        uci::run(parameters);
    }
}

struct DatagenCommand;
// TODO: refactor cli - split uci / cli / datagon into separate mods
impl CliCommand for DatagenCommand {
    fn run(&self, parameters: &[&str]) {
        if parameters.len() < 3 {
            write_line(
                "Usage: datagen <output.binpack> <number_of_games> <opening_book.fen> [depth] [threads]",
            );
            return;
        }
        let output_path = parameters[0];
        let num_games = match parameters[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                write_line("Error: invalid number of games");
                return;
            }
        };
        let book_path = parameters[2];
        let depth = if parameters.len() > 3 {
            match parameters[3].parse::<u8>() {
                Ok(d) => d,
                Err(_) => {
                    write_line("Error: invalid depth");
                    return;
                }
            }
        } else {
            8
        };

        let threads = if parameters.len() > 4 {
            match parameters[4].parse::<usize>() {
                Ok(t) => t,
                Err(_) => {
                    write_line("Error: invalid thread count");
                    return;
                }
            }
        } else {
            std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4)
        };

        datagen::run(output_path, num_games, book_path, depth, threads);
    }
}

pub fn run() {
    let mut commands: HashMap<&str, Box<dyn CliCommand>> = HashMap::new();
    commands.insert("info", Box::new(InfoCommand));
    commands.insert("uci", Box::new(UciCommand));
    commands.insert("datagen", Box::new(DatagenCommand));

    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => continue,
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
