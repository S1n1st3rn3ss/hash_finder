mod tests;
mod lib;

use clap::Parser;
use crate::lib::run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'N')]
    number: String,
    #[arg(short = 'F')]
    find: String,
}
fn main() {
    let cli = Cli::parse();
    let zero_count = cli.number.parse::<i32>().expect("argument N should be given");
    let hash_count = cli.find.parse::<i32>().expect("argument F should be given");

    run(zero_count, hash_count);
}

