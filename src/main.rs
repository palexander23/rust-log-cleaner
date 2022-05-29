use clap::Parser;

use log_cleaner::run_script;

// Demo program to greet people
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ArgList {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// The number of greetings to print
    #[clap(short, long)]
    count: u8,
}

fn main() {
    let args = ArgList::parse();

    run_script(args.name, args.count);
}
