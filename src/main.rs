//use std::{env, process};
//
//use minicat::Config;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show number lines
    #[arg(short, long)]
    number: bool,
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    //
    //let config = Config::build(&args).unwrap_or_else(|err| {
    //    eprintln!("Problem parsing arguments: {err}");
    //    process::exit(1);
    //});
    //
    //if let Err(e) = minicat::run(config) {
    //    println!("Application error: {e}");
    //    process::exit(1);
    //}
    let args = Args::parse();
    println!("{}", args.number);
}
