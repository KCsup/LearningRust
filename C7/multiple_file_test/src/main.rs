mod signup;

use signup::{util::SignUp, OtherStructLol};

fn main() {
    let s = SignUp { count: 4 };

    let other_struct = OtherStructLol {
        name: "Name".to_string(),
        other_attr: 42,
    };
}
