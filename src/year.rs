use chrono::Datelike;


fn main() {
    let today = chrono::Local::now().naive_local();
    // println!("{:?}", today);

    let first = chrono::NaiveDate::
        from_ymd_opt(today.year(), 1, 1).unwrap()
        .and_hms_opt(0, 0, 0).unwrap();
    // println!("{:?}", first);

    // let next_year_first = chrono::NaiveDate::from_ymd_opt(first.year() + 1, 1, 1).unwrap();
    let next_year_first = first.with_year(first.year() + 1).unwrap();
    // println!("{:?}", next_year_first);

    let duration_year = next_year_first - first;
    let num_sec = duration_year.num_seconds();
    // println!("{:?}", num_sec);

    let diff = today - first;
    let sec_passed = diff.num_seconds();
    // println!("{:?}", sec_passed);

    println!("{:.4}%", 100.0 * sec_passed as f64 / num_sec as f64);

    let percent_passed = sec_passed*100/num_sec;

    for _ in 0..percent_passed {
        print!("x");
    }
    for _ in percent_passed..100 {
        print!(".");
    }
    println!();

}