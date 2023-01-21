// use rand::seq::index;

fn main() {
    use rand::{thread_rng, Rng};
    // use rand::distributions::{Distribution, Uniform};

    // let between = Uniform::from(0..7_500_000);
    let mut rng = thread_rng();

    let mut population = vec![false; 7_500_000];
    // let mut population = vec![true; 7_500_000];

    population[0] = true;

    for _ in 0..20 {
        let mut new_infection: Vec<usize> = vec![];
        for p in &population {
            if *p {
                for _ in 0..10 {
                    let n = rng.gen_range(0..population.len() as u32) as usize;
                    // let n = between.sample(&mut rng);
                    // let n = 10;
                    if !population[n] {
                        new_infection.push(n);
                    }
                }
            }
        }
        new_infection.sort();
        new_infection.dedup();

        for i in new_infection {
            population[i] = true;
        }

        let n = population.iter().filter(|&x| *x).count();
        println!("n: {}", n);
    }

}