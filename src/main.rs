mod tests;

use std::{env, thread};
use std::sync::mpsc;
use sha256::{digest};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'N')]
    number: String,
    #[arg(short = 'F')]
    find: String,
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let cli = Cli::parse();
    // let mut args_hashmap: HashMap<String, String>;
    // slightly basic, but avoids adding
    // args_hashmap.insert(args[1], args[2]);
    let zero_count = cli.number.parse::<i32>().expect("argument N should be given");
    let hash_count = cli.find.parse::<i32>().expect("argument F should be given");

    run(zero_count, hash_count);
}
// Creates thread to run hash finder in, handles Results received from Sender
fn run(zero_count: i32, hash_count: i32) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || find_relevant_hashes(zero_count, hash_count, tx));

    for received in rx {
        match received {
            Ok((int, string)) => println!("{}, {}", int, string),
            Err(e) => println!("{}", e)
        }
    }
    println!("done!")
}
// Runs through all i32 values to find hashes with needed amount of zeros
// Possible errors: Returns a String if not enough hashes are found
fn find_relevant_hashes(zero_count: i32, hash_count: i32, sender: mpsc::Sender<Result<(i32, String), String>>) {
    let mut counter: i32 = hash_count;
    let zero: i32 = zero_count;
    const MAX_I32: i32 = i32::MAX;
    for i in 0..MAX_I32 {
        let hash = create_hash_digest(i);
        if hash.ends_with(&"0".repeat(zero as usize)) {
            counter -= 1;
            sender.send(Ok((i, hash.to_owned()))).unwrap();
        }
        if counter == 0 {
            break;
        }
    }
    // return an Err() in case not all hashes are found
    if counter != 0 {
        // receiver should never drop so .unwrap() is safe
        sender.send(Err(format!("Didn't find all {} hashes, found {}", hash_count, hash_count - counter))).unwrap();
    }
}
fn create_hash_digest(number: i32) -> String {
    // sha256 crate doesn't implement digest() for integer types directly so typecasting is needed
    let number_string = number.to_string();
    digest(number_string)
}