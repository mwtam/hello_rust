use chrono::Datelike;


fn main() {
    let today = chrono::Local::now();
    // println!("{:?}", today);

    let first = chrono::NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();
    // println!("{:?}", first);

    let next_year_first = chrono::NaiveDate::from_ymd_opt(first.year() + 1, 1, 1).unwrap();
    // println!("{:?}", next_year_first);

    let duration_year = next_year_first - first;
    let num_days = duration_year.num_days();
    // println!("{:?}", num_days);

    // let diff = first - chrono::Duration::days(1);
    let diff = today.naive_local().date() - first;
    let days_passed = diff.num_days();
    // println!("{:?}", days_passed);

    println!("{:.2}%", 100.0 * days_passed as f64 / num_days as f64);

    for _ in 0..days_passed {
        print!("x");
    }
    for _ in days_passed..num_days {
        print!(".");
    }
    println!();

}