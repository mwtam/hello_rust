use itertools::Itertools;

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
}