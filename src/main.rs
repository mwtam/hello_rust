use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    for i in 0..4 {
        for j in i+1..4 {
            println!("{i}, {j}");
        }
    }

    println!("----");

    for v in (0..4).combinations(2) {
        let i = v[0];
        let j = v[1];
        println!("{i}, {j}");
    }

    println!("----");
    // let v = (0..4).combinations(2).collect::<Vec<_>>();
    // let v = vec![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];
    // v.into_par_iter();
    // println!("{v:?}");
}