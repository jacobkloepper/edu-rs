use std::collections::HashMap;

fn fibonacci_mem(n: usize, memory: &HashMap<usize,usize>) -> usize {
    match n {
        0 => return 0,
        1 => return 1,
        2 => return 1,
        _ => (),
    }

    match memory.get(&n) {
        Some(&val) => return val,
        _ => (),
    }

    return fibonacci_mem(n-1, &memory) + fibonacci_mem(n-2, &memory);
}

fn fibonacci(n: usize) -> usize {
    let mut memory = HashMap::new();

    memory.insert(n, fibonacci_mem(n, &memory));

    return fibonacci_mem(n-1, &memory) + fibonacci_mem(n-2, &memory);
}

fn main() {
    const N: usize = 100;

    for i in 0..=N {
        println!("F_{} = {}", i, fibonacci(i));
    }
}
