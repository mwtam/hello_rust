fn main() {
    println!("Hello");
    // Make a 7.5 M list of people
    //
    // Let groups of 3-4 closely connected (infection rate = 1)
    // to simulate families.
    //
    // Let groups of 10-12 mildly connected (infection rate = 0.5)
    // to simulate families.
    //
    // For each person, let them loosely connect to
    // random 100 people (rate = 0.05) in each iteration,
    // to simulate connections like riding on the same bus.
    // 
    // Remember how long they are infected.
    // Heal them after 10 iterations.
    // To simulate recovery
    //
    // Vaccine? Add a factor to decrease the rates.
    // Say, if vaccinated, the rate is cut half.
    // Same for those recovered.
    //
    // Isolation of the infected
    // Make them very unlikely to get picked.
    // A reject picking factor.
    // Not re-using the vaccine factor?

    // Measure how many iterations it takes to infect everyone at least once.
}