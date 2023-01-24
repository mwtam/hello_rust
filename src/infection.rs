// use rand::seq::index;

fn main() {
    use rand::{thread_rng, Rng};
    // use rand::distributions::{Distribution, Uniform};

    // let between = Uniform::from(0..7_500_000);
    let mut rng = thread_rng();

    let infectious_round = 5;

    let mut population = vec![0; 7_500_000];
    // let mut population = vec![true; 7_500_000];

    population[0] = infectious_round;
    let n_population = population.len();

    for _ in 0..40 {
        let mut new_infection: Vec<usize> = vec![];
        new_infection.reserve(population.len());
        for (i, p) in population.iter().enumerate() {
            // if !*p {
            //     for _ in 0..0 {
            //         let n = (rng.gen_range(0..5) as usize + i) % n_population;
            //         if population[n] {
            //             new_infection.push(i);
            //         }
            //     }
            //     for _ in 0..1 {
            //         let n = (rng.gen_range(0..n_population) as usize + i) % n_population;
            //         if population[n] {
            //             new_infection.push(i);
            //         }
            //     }
            // }
            if *p > 0 {
                for _ in 0..1 {
                    let n = (rng.gen_range(0..5) as usize + i) % n_population;
                    if population[n] == 0 {
                        new_infection.push(n);
                    }
                }
                for _ in 0..1 {
                    let n = rng.gen_range(0..n_population);
                    if population[n] == 0 {
                        new_infection.push(n);
                    }
                }

            }
        }

        for p in population.iter_mut() {
            if *p > 0 {
                *p -= 1;
            }
        }
        new_infection.sort();
        new_infection.dedup();

        for i in new_infection {
            population[i] = infectious_round;
        }

        let n = population.iter().filter(|&x| *x > 0).count();
        println!("n: {}", n);
    }

}