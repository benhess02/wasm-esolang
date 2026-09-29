use interpreter::Interpreter;

use std::{path::PathBuf, thread, time::Duration};

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Args)]
struct Run {
    file: PathBuf,
    #[arg(short = 'd', long = "debug", default_value_t = false)]
    debug: bool,
    #[arg(short = 'a', long = "animate", default_value_t = false)]
    animate: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run(Run),
}

fn print_char(b: u8) {
    print!("{}", b as char);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(r) => {
            run(r).expect("must run");
        }
    }
}

fn run(r: Run) -> Result<(), String> {
    let _s: String = std::fs::read_to_string(r.file).map_err(|e| format!("{e}"))?;
    let mut interpreter = Interpreter::new(print_char);
    let mut state = interpreter.interpret(&_s);

    if r.debug {
        while state.code_ptr < state.chars.len() {
            println!("{}", _s);
            println!("{}^", " ".repeat(state.code_ptr));
            println!(
                "Memory location: {}, byte at pointer: {}",
                state.interpreter.mem_ptr, state.interpreter.memory[state.interpreter.mem_ptr]
            );
            println!();
            state.step();
            if r.animate {
                thread::sleep(Duration::from_millis(100));
            }
        }
    } else {
        state.run();
    }
    Ok(())
}
