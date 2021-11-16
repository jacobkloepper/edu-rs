fn main() {
    const N: usize = 1000;
    let mut primes = vec![true; N];
    
    // we are doubling the index as the numerical value. base cases: 0 and 1 are non-prime
    primes[0] = false;
    primes[1] = false;

    // do the sieve
    let mut k: usize = 2;
    for p in 2..N {
        if primes[p] == false {
            continue;
        }

        while k*p < N {
            primes[k*p] = false;
            k += 1;
        }
        k = 2;
    }

    
    // pretty print
    println!("There are {} primes less than {}:", primes.iter().filter(|&val| *val == true).count(), N);

    for (i,val) in primes.iter().enumerate() {
        if *val && i > 2 {
            print!(", ");
        }
        match i {
            0..=1 => (),
            2 => {
                if *val {
                    print!("{}", i);
                }
            }
            3..=N => {
                if *val {
                    print!("{}", i);
                }
            }
            _ => (),
        }
    }
    println!("");
}
