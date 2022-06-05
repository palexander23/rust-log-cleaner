use clap::Parser;

use log_cleaner::run_script;

// Demo program to greet people
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ArgList {
    /// The name of the existing log file you want to convert
    #[clap(short, long)]
    input_file: String,

    /// The name you want to give the output csv file
    #[clap(short, long)]
    output_file: String,

    /// The list of characters that signify comments to be removed
    #[clap(short, long, default_value = "#")]
    deliminator_str: String,
}

fn main() {
    let args = ArgList::parse();

    run_script(&args.input_file, &args.output_file, &args.deliminator_str);
}
