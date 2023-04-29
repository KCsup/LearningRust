fn main() {
    let coin_var = Coin::Quarter(State::AmongUs);

    println!("Coin value: {}", get_coin_value(&coin_var));
}

#[derive(Debug)]
enum State {
    AmongUs,
    AmongUsAgain,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn get_coin_value(c: &Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("Quarter from {:?}", s);
            25
        },
    }
}
