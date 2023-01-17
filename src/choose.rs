
use std::env;
use std::process::exit;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2
    {
        println!("Usage: {} <options>", args[0]);
        exit(1);
    }

    let mut rng = rand::thread_rng();

    let i = rng.gen_range(1..args.len());

    println!("{}", args[i]);
}