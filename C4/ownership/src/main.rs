fn main() {
//    let s1 = [1, 4, 5];
//    let s2 = s1;
//    println!("{:?}", s1);
    let s1 = String::from("e");
    let s3: &String = &s1;

    println!("{}", s1);
}

fn returns_string(string_input: String) -> String {
    string_input
}
