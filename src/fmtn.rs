use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        println!("Usage: {} 123456.7", args[0]);
        exit(1);
    }

    let s = &args[1];

    let mut ss = String::with_capacity(s.len());
    let mut dot_i = 0;

    let mut dot_found = false;

    for c in s.chars() {
        if c.is_numeric() {
            ss.push(c);
            if !dot_found {
                dot_i += 1;
            }
        }
        if c == '.' {
            dot_found = true;
        }
    }

    let mut next_comma = dot_i % 3;
    if next_comma == 0 {
        next_comma = 3;
    }
    for c in ss.chars() {
        print!("{c}");

        dot_i -= 1;
        if dot_found && dot_i == 0 {
            print!(".");
        }

        next_comma -= 1;
        if next_comma == 0 && dot_i > 0 {
            print!(",");
            next_comma = 3;
        }
    }
    print!("\n");
}