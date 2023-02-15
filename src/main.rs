
fn main() {
    loop_with_break_value();
}


fn loop_with_break_value(){
    let mut i = 140;
    let s = loop {
        if i % 13 == 0 {
            break i;
        }
        i += 1;
    };
    println!("{s}");
}