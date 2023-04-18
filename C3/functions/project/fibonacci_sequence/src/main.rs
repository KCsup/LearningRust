fn main() {
    for i in 0..30 {
        println!("F {}: {}", i, fibonacci(i));
    }
}

fn fibonacci(n: usize) -> u32 {
    let solutions = [0, 1];

    if n < 2 { return solutions[n]; }
    
    let mut f1 = solutions[1];
    let mut f2 = solutions[0];

    for i in 2..(n + 1) {
        let f = f1 + f2;
        f2 = f1;
        f1 = f;

        if i == n { return f; }
    }

    0
}
