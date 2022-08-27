pub fn factors(mut n: u64) -> Vec<u64> {
    let mut outcome = vec![];
    let mut divisor = 2;
    while n > 1 {
        if n % divisor == 0 {
            outcome.push(divisor as u64);
            n /= divisor;
            divisor = 2;
        } else {
            divisor += 1;
        }
    }
    outcome
}