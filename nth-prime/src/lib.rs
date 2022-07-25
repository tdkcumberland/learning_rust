pub fn nth(n: u32) -> u32 {
    // always use the next odd number and check if they have a modulus of zero with any of the existing primes found
    let mut primes: Vec<u32> = vec![2,3];
    let mut size = primes.len();
    while size <= n as usize {
        // reinitialize the next prime
        // after exiting the inner while and the size of primes is still not met condition
        let mut next = primes.last().unwrap() + 2;
        while primes.iter().any(|p| next % p == 0) {
            // always odd
            next += 2;
        }
        primes.push(next);
        size = primes.len();
    }
    primes[n as usize]
}
