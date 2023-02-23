use rayon::prelude::*;
use rand::{thread_rng, Rng, rngs::ThreadRng};

fn main() {
    // for i in 0..4 {
    //     for j in i+1..4 {
    //         println!("{i}, {j}");
    //     }
    // }

    // println!("----");

    // for v in (0..4).combinations(2) {
    //     let i = v[0];
    //     let j = v[1];
    //     println!("{i}, {j}");
    // }

    let mut rng = thread_rng();

    let n = 6;
    (0..n)
    .into_par_iter()
    .for_each(
        |i| (i+1..n)
        .into_par_iter()
        .for_each(
            |j|
             println!("Hello {} {}", i, j)
        )
    );
}