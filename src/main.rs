mod lexer;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use structopt::StructOpt;

/// A command that our CLI can process
#[derive(Debug, StructOpt)]
enum Command {
    /// Print the tokens produced by the lexer.
    Lex {
        /// The file containing Wahlbergdown code you want to lex.
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        input_file: PathBuf,
    },
    /// Print the AST produced by the parser.
    Parse {
        /// The file containing Wahlbergdown code you want to parse.
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        input_file: PathBuf,
    },
    // Run a file.
    Run {
        /// The file containing Wahlbergdown code you want to run.
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        input_file: PathBuf,
    },
}

fn run(input_file: &Path) {
    let src = fs::read_to_string(input_file).expect("failed to read input file");
    print!("{}", src)
}

fn main() {
    let args = Command::from_args();
    match args {
        Command::Lex { .. } => eprintln!("unimplemented"),
        Command::Parse { .. } => eprintln!("unimplemented"),
        Command::Run { input_file } => run(&input_file),
    }
}
