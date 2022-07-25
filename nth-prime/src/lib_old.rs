//worked but took 300s plus to finish
pub fn nth(n: u32) -> u32 {
    // algo sources: https://www.baeldung.com/cs/prime-number-algorithms
    // using the least efficient but easiest to implement
    // Sieve of Eratosthenes
    let mut primes: Vec<u128> = vec![2,3];
    let mut non_primes: Vec<u128> = vec![4,9];
    let mut size = primes.len();
    let mut next:u128 = 4;
    while size <= n as usize {
        // println!("{:?} {:?}", next, size);
        non_primes.push(next.pow(2));
        
        if !non_primes.contains(&next) {
            let mut flag = true;
            for p in &primes {
                if next % p == 0 {
                    flag = false;
                    break
                }
            }
            if flag {primes.push(next)} else {non_primes.push(next)}
        }
        next +=1;
        size = primes.len();
    }
    // println!("{:?} {:?} {}", non_primes, primes, size);
    *primes.get(n as usize).unwrap() as u32
        
}
