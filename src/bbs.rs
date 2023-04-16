#![allow(non_snake_case)]

use num::{BigUint, Integer, One};
use num_prime::nt_funcs::is_prime;
pub struct BBS {
    p: BigUint,
    q: BigUint,
    M: BigUint,
    seed: BigUint,
}

impl BBS {
    pub fn new(p_string: &[u8], q_string: &[u8], seed_string: &[u8]) -> Result<BBS, Vec<String>> {
        let p = BigUint::parse_bytes(p_string, 10).unwrap();
        let q = BigUint::parse_bytes(q_string, 10).unwrap();
        let seed = BigUint::parse_bytes(seed_string, 10).unwrap();

        let mut errors = Self::check_primes_validity(&p, &q);
        if !errors.is_empty() {
            return Err(errors);
        }

        let M = &p * &q;

        if M.gcd(&seed) != BigUint::one() {
            errors.push(format!("{} not coprime with {}", seed, M));
            return Err(errors);
        }
        Ok(BBS { p, q, M, seed })
    }

    fn check_validity(x: &BigUint) -> Result<bool, String> {
        if x % &BigUint::from(4u32) != BigUint::from(3u32) {
            return Err(format!("{} % 4 != 3", x));
        }
        if !is_prime(x, None).probably() {
            return Err(format!("{} not prime", x));
        }

        return Ok(true);
    }

    fn check_primes_validity(p: &BigUint, q: &BigUint) -> Vec<String> {
        // TODO: should be safe primes with a small gcd((p-3)/2, (q-3)/2) (this makes the cycle length large).
        let mut errors = Vec::new();
        match Self::check_validity(p) {
            Err(msg) => errors.push(msg),
            _ => (),
        }
        match Self::check_validity(q) {
            Err(msg) => errors.push(msg),
            _ => (),
        }
        errors
    }

    pub fn next(&mut self) -> bool {
        self.seed = self.seed.modpow(&BigUint::from(2u32), &self.M);
        // print!("new seed: {:?}\n", self.seed);
        self.seed.bit(0)
    }

    pub fn get_n_bits(&mut self, n: usize) -> String {
        let mut result = String::with_capacity(n);
        for _ in 0..n {
            let bit = if self.next() { '1' } else { '0' };
            result.push(bit);
        }
        result
    }

    pub fn get_p(&self) -> &BigUint {
        &self.p
    }

    pub fn get_q(&self) -> &BigUint {
        &self.q
    }

    pub fn generate_u8(&mut self) -> u8 {
        let bit_string = self.get_n_bits(8);
        u8::from_str_radix(&bit_string, 2).unwrap()
    }

    pub fn generate_u16(&mut self) -> u16 {
        let bit_string = self.get_n_bits(16);
        u16::from_str_radix(&bit_string, 2).unwrap()
    }

    pub fn generate_u32(&mut self) -> u32 {
        let bit_string = self.get_n_bits(32);
        u32::from_str_radix(&bit_string, 2).unwrap()
    }

    pub fn generate_u64(&mut self) -> u64 {
        let bit_string = self.get_n_bits(64);
        u64::from_str_radix(&bit_string, 2).unwrap()
    }

    pub fn generate_u128(&mut self) -> u128 {
        let bit_string = self.get_n_bits(128);
        u128::from_str_radix(&bit_string, 2).unwrap()
    }
}

impl Default for BBS {
    fn default() -> BBS {
        let p = BigUint::parse_bytes(
            b"81282214694636016670763621761800109247752388157581129517313895767689154087487",
            10,
        )
        .unwrap();

        let q = BigUint::parse_bytes(
            b"65810608086241557256723687330212888597389852144534660261023144057762936489759",
            10,
        )
        .unwrap();
        let M = &p * &q;
        let seed = BigUint::parse_bytes(b"42", 10).unwrap();
        BBS { p, q, M, seed }
    }
}
