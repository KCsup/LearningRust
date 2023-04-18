const GIFTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtledoves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

const DAYS: [&str; 12] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh",
    "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"
];

fn main() {
    println!("{}", get_lyrics());
}

fn get_lyrics() -> String {
    let mut result = "".to_string();

    for i in 0..12 {
        let mut day_string = format!("On the {} day of Christmas, my true love sent to me", DAYS[i]);
        for j in (0..(i + 1)).rev() {
            day_string = format!("{}\n{}", day_string, GIFTS[j]);
        }
        result = format!("{}\n\n{}", result, day_string);
    }

    result.to_string()
}

// :(
