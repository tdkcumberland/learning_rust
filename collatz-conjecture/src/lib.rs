pub fn collatz(n: u64) -> Option<u64> {
    match n {
        1 => Some(0),
        0 => None,
        _ => iterate_until_one(n)
    }
}

fn iterate_until_one(n: u64) -> Option<u64> {
    let mut temp = n;
    let mut steps = 0;
    while temp != 1 {
        if temp % 2 == 1 {
            let (m, b_mul) = temp.overflowing_mul(3);
            if b_mul {
                return None
            } else {
                let (a, b_add) = m.overflowing_add(1);
                if b_add {
                    return None
                } else {
                    temp = a;
                    steps += 1;
                }
            }
        } else {
            temp = temp/2;
            steps += 1;
        }
    }
    Some(steps)
}
