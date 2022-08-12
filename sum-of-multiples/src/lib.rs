pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0
    }
    let big_factor = factors.iter().fold(1, |acc, n| {if n > &0 {acc *n} else { acc * 1}});
    let vec: Vec<u32> = (0..limit).map(u32::from).collect();
    let output = vec.iter().filter(|n| check_factor(n, factors, &big_factor)).fold(0, |acc, n| acc + n);
    output
}

fn check_factor(n:&u32, factors: &[u32], big_factor: &u32) -> bool {
    for i in factors {
        if i > &0 && n % i == 0 {
            return true
        }
    }
    return n % big_factor == 0
}