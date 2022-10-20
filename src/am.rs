
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        println!("Usage: {} https://www.amazon.com/Something/dp/XXXXXXXX/some_rubbish", args[0]);
        exit(1);
    }

    let url = &args[1];

    let mut iter = url.split("/dp/");

    iter.next(); // Throw away what comes before "/dp/"

    let after_dp = match iter.next() {
        None => {println!("Not Found: Expects /dp/ in the URL"); exit(2)},
        Some(s) => s,
    };

    let mut iter = after_dp.split("/");


    let id = iter.next().unwrap(); // Something must exist, unwrap directly.

    println!("https://www.amazon.com/dp/{id}");
}