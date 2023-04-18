fn main() {
    let out = fahrenheit_to_celcius(3.0);
    println!("{} Degrees Celcius", out);
}

fn fahrenheit_to_celcius(f: f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
}
