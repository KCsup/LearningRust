fn main() {
    let num = Some(5);

    println!("{}", plus_one(&num));

    let no_num: Option<i32> = None;
    println!("Result from no number: {}", plus_one(&no_num));
}

fn plus_one(x: &Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}
