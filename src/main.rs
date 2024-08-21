use std::process;

//use minicat::Config;
use clap::Parser;
use minicat::{Args, Config};

fn main() {
    //let args: Vec<String> = env::args().collect();
    let args = Args::parse();
    //println!("{}", args.line_num);

    //let files = &args.files;
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minicat::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    //let config = Config::build(&args).unwrap_or_else(|err| {
    //    eprintln!("Problem parsing arguments: {err}");
    //    process::exit(1);
    //});
    //
    //if let Err(e) = minicat::run(config) {
    //    println!("Application error: {e}");
    //    process::exit(1);
    //}
}
