fn main() {
    let mut string_input = String::from("this is a sentance");
    let word = first_word(&string_input);

    println!("{}", word);

    string_input.clear(); // can clear since the reference to
                          // 'string_input' is released after the last
                          // reference
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

