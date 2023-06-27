use clap::Parser;
use std::fs;
pub mod compiler;
pub mod interpreter;
pub mod optimizer;
pub mod parser;
pub mod runner;
pub mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interpret: bool,
    #[arg(short, long)]
    no_optimization: bool,
    #[arg(short, long)]
    print_ir: bool,
    file: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.file).expect("Could not read file");
    runner::run(&code, runner::DEFAULT_COMPILE_DEPTH);
}
