fn main() {
    // this sieve gets a list of the odd primes below 2N+1.
    // Therefore, we manually add 2 to the count.
    const N: usize = 1000;
    let mut primes = vec![true; N];

    // base case: 0 -> 2(0)+1 = 1 is nonprime
    primes[0] = false;
    
    // do the sieve
    let mut j: usize = 1;
    
    // sieve condition: i+j+2ij <= N.
    // edge: i=1 -> 3j+1 <= N
    while 3*j+1 < N {

        for i in 1..=j {
            if i+j+2*i*j >= N {
                break;
            }

            primes[i+j+2*i*j] = false;
        }

        j += 1;
    }
    
    // pretty print
    // we add one to the count for 2
    println!("There are {} primes less than {}:", primes.iter().filter(|&val| *val == true).count()+1, 2*N+1);

    // base case: this sieve does not count 2.
    print!("2, ");
    for (i,val) in primes.iter().enumerate() {
        if *val && i > 1 {
            print!(", ");
        }
        match i {
            1..=N => {
                if *val {
                    print!("{}", 2*i+1);
                }
            }
            _ => (),
        }
    }
    println!("");
}
