mod signup;

use signup::{util::SignUp, OtherStructLol};

fn main() {
    let s = SignUp { count: 4 };

    let other_struct = OtherStructLol {
        name: "Name".to_string(),
        other_attr: 42,
    };

    let other_other_struct = OtherStructLol::new("Name Here");

    println!("{}", other_other_struct.other_attr);
}
