pub fn square(s: u32) -> u64 {
    if (s >= 1) & (s <= 64) {
        (2 as u64).pow(s-1)
    } else {
        panic!("Square must be between 1 and 64")
    }
    
}

pub fn total() -> u64 {
    let v = 1..=64;
    let t:u128 = v.map(|n| (2 as u128).pow(n)).sum();
    (t+1) as u64
}
