use std::ops::Add;

use rand::Rng;

fn main() {
    // test();
    // test_rand();
    // test_loop();
    // test_ownership();
    // test_vec();
    // test_magic();
    // test_take_own();
    test_string();
}

fn test_string() {
    let s = "Fish mouth Opening";
    let t: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    println!("{s}");
    println!("{t}");
}

fn f(v: &[String]) {
    for x in v {
        println!("x = {x}");
    }
}

fn test_take_own() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("Fish");
    let mut v:Vec<String> = vec![];
    v.push(s1);
    v.push(s2);
    v.push(s3);

    f(&v);
}

fn test_magic() {
    // let n:i32 = (2..=4).product();
    let n:i32 = (1..=4).sum();
    println!("factorial {}", n);

    let v = vec![1,2,3,4,5];
    let s = (&v).into_iter().sum::<i32>();
    println!("sum: {}", s);

    for x in v {
        println!("x = {}", x)
    }
}

fn test_vec() {
    let v = vec![1,2,3,4,5,6,7,8,9];

    // let even_numbers = v
    // .into_iter()
    // .filter(|n| n % 2 == 0)
    // .collect::<Vec<_>>();

    for x in (&v).into_iter().filter(|&y| y % 2 == 0) {
        println!("x = {}", x)
    }

    println!("v.len {}", v.len());

    // for x in v {
    //     println!("x = {}", x)
    // }

}

fn test_ownership() {
    // let x = 5;
    // let y = x;

    let x = String::from("Fish mouth opening");
    let y = x.as_str();
    // let y = x.clone();

    println!("x = {}, y = {}", x, y);

    // let v = vec![1,2,3,4,5,6,7,8,9];
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("Fish");
    let mut v:Vec<String> = vec![];
    v.push(s1);
    v.push(s2);
    v.push(s3);

    // println!("s1 = {}", s1); // No more. Moved.
    let s11 = &v[0];
    println!("s1 = {}", v[0]);
    

    // let even_numbers = v
    // .into_iter()
    // .filter(|n| n % 2 == 0)
    // .collect::<Vec<_>>();

    let v2 = (&v)
    .into_iter()
    .filter(|x| x.len() > 4)
    .collect::<Vec<_>>();

    for x in &v {
        println!("x1 = {}", x)
    }

    for x in &v2 {
        println!("x3 = {}", x)
    }

    for x in (&v).into_iter().filter(|&y| y.len() == 4) {
        println!("x2 = {}", x)
    }

    println!("v.len {}", v.len());
}

fn test() {
    println!("Hello, world!");

    let x: i32 = 42;
    println!("Hello, {}!", x);

    let r = (0..).contains(&39);
    println!("{}", r);

    let v = vec! [1,6,7,8,9,45,5,7,2,7,8];
    for x in &v {
        println!("x = {}", x)
    }
    let has_2 = v.contains(&2);
    println!("has 2 {}", has_2);

    let n = 10;
    println!("v[{}] = {}", n, v[n]);
}

fn test_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn test_rand() {
    let mut rng = rand::thread_rng();
    let r_n: i32 = rng.gen();
    println!("rand {}", r_n);
}