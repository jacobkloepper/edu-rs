fn fibonacci(n: usize) -> usize {
    match n {
        0 => return 0,
        1 => return 1,
        2 => return 1,
        _ => (),
    }

    return fibonacci(n-1) + fibonacci(n-2);
}

fn main() {
    const N: usize = 100;

    for i in 0..=N {
        println!("F_{} = {}", i, fibonacci(i));
    }
}
