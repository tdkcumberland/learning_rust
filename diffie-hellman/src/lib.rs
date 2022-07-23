pub fn private_key(p: u64) -> u64 {
    p%200-2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow_mod(a, p)

}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow_mod(a, p)
}

trait PowMod {
    fn pow_mod(&self, power: u64, modulus: u64) -> u64;
}

impl PowMod for u64 {
    fn pow_mod(&self, power: u64, modulus: u64) -> u64 {
        // if modulus is one, always zero
        if modulus == 1 {
            return 0;
        } else {
            // initialize the results to 1
            // initialize the base to original base value
            let mut r = 1;
            let mut p = power;
            let mut b = *self;
            while p > 0 {
                if p %2 == 1 {
                    r = (r * b) % modulus;
                } 
                b = (b * b) % modulus;
                
                p >>= 1;
            }
            return r;
        }
    }
}