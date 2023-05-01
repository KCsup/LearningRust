fn main() {
    let some_num = Some(4);

    println!("{:?}", add_one(some_num));

    println!("{:?}", some_num);
}

fn add_one(input: Option<i8>) -> Option<i8> {
    if let Some(num) = input {
        return Some(num + 1);
    }

    None
}
