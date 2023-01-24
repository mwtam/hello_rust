// use rand::seq::index;

fn main() {
    use rand::{thread_rng, Rng};
    // use rand::distributions::{Distribution, Uniform};

    // let between = Uniform::from(0..7_500_000);
    let mut rng = thread_rng();

    let infectious_round = 5;
    let immunized_round = 20;

    // let mut population = vec![0; 7_500_000];

    // Make all people has immunity
    let mut population = vec![immunized_round; 7_500_000];

    // Infect the person 0
    population[0] = immunized_round + infectious_round;

    let n_population = population.len();

    // Make some people no immunity
    for i in 1..(n_population/100 as usize) {
        population[i] = 0;
    }

    for _ in 0..100 {
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
            if *p > immunized_round {
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
            population[i] = immunized_round + infectious_round;
        }

        let n_immunized = population.iter().filter(|&x| *x > 0).count();
        let n_infectious = population.iter().filter(|&x| *x > immunized_round).count();
        println!("{}\t{}", n_immunized, n_infectious);
    }

}