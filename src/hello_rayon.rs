use rayon::prelude::*;

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn n_prime(n: i32) -> i32 {
    let mut n_of_p = 0;
    for i in 2..n {
        if is_prime(i) {
            n_of_p+=1;
        }
    }
    n_of_p
}

fn n_prime_rayon(n: i32) -> i32 {
    (2..n).into_par_iter()
        .filter(|&i| is_prime(i))
        .count() as i32
}

fn main() {
    let n_range = 300000;

    // let n = n_prime(n_range);
    // Answer = 25997
    // Time = 14.29

    let n = n_prime_rayon(n_range);
    // Answer = 25997
    // Time = 3.41

    println!("{n}");
}