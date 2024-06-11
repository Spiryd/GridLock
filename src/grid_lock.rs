use rand::{
    distributions::{Distribution, Uniform},
    Rng, SeedableRng,
};
use rand_hc::Hc128Rng;

use crate::z::{ChiDistribution, Z};

use crate::bitvec::{Bit, BitVec};

/// security parameter n
pub const N: usize = 64;

/// p should be a prime where p >= 2 and n^2 < p < 2n^2
pub const P: usize = 4099;

/// m = (1 + ε)(n + 1) log p for some arbitrary constant ε >= 0
/// e.g. ε = 0.1
pub const M: usize = 594;

/// GridLock is a lattice-based cryptographic scheme
/// based on the Learning With Errors (LWE) problem
/// as described in [arXiv:2401.03703](https://arxiv.org/abs/2401.03703)
#[derive(Debug)]
pub struct GridLock {
    n: usize,
    m: usize,
    p: usize,
    uniform: Uniform<Z>,
    rng: Hc128Rng,
}

impl Default for GridLock {
    fn default() -> Self {
        Self::new()
    }
}

impl GridLock {
    /// Create a new [GridLock] instance
    pub fn new() -> GridLock {
        GridLock {
            n: N,
            m: M,
            p: P,
            uniform: Uniform::new_inclusive(Z::new(0), Z::new(P - 1)),
            rng: Hc128Rng::from_entropy(),
        }
    }
    /// Generate a private key
    pub fn gen_secret_key(&mut self) -> Vec<Z> {
        self.gen_vector()
    }
    /// Generate a public key
    pub fn gen_public_key(&mut self, s: &[Z]) -> Vec<(Vec<Z>, Z)> {
        // Generate m vectors a_1, ..., a_m in Z^n_p independently from the uniform distribution
        let a = (0..self.m).map(|_| self.gen_vector()).collect::<Vec<_>>();
        //choose elements e_1, ..., e_m ∈ Z_p independently according to chi
        let mut e = Vec::new();
        let mut chi = ChiDistribution::new(&mut self.rng);
        for _ in 0..self.m {
            e.push(chi.get());
        }
        // b_i = <a_i, s> + e_i
        let mut b = Vec::new();
        for i in 0..self.m {
            let mut sum = Z::new(0);
            for j in 0..self.n {
                sum += a[i][j] * s[j];
            }
            b.push(sum + e[i]);
        }
        // return (a_i, b_i)^m_i=1
        a.iter()
            .cloned()
            .zip(b.iter().cloned())
            .collect::<Vec<(Vec<Z>, Z)>>()
    }
    /// Generate a vector of random uniform elements in Z_p
    fn gen_vector(&mut self) -> Vec<Z> {
        (0..self.n)
            .map(|_| self.uniform.sample(&mut self.rng))
            .collect()
    }
    /// Encrypt a message
    pub fn encrypt(&mut self, public_key: &[(Vec<Z>, Z)], message: BitVec) -> Vec<(Vec<Z>, Z)> {
        let mut ciphertext = Vec::new();
        let mut s = Vec::new();
        // choose a random subset S of [m]
        for i in 0..self.m {
            if self.rng.gen_bool(1.0 / self.m as f64) {
                s.push(i);
            }
        }
        for bit in message {
            // Encription is (sum_{i in S} a_i, sum_{i in S} b_i + p/2 * bit)
            let mut encrypted_bit = (vec![Z::new(0); self.n], Z::new(0));
            for i in &s {
                for j in 0..self.n {
                    encrypted_bit.0[j] += public_key[*i].0[j];
                }
                encrypted_bit.1 += public_key[*i].1;
            }
            if bit == Bit::One {
                encrypted_bit.1 += Z::new(self.p / 2);
            }
            ciphertext.push(encrypted_bit);
        }
        ciphertext
    }
    /// Decrypt a message
    pub fn decrypt(&self, secret_key: &[Z], ciphertext: &Vec<(Vec<Z>, Z)>) -> BitVec {
        let mut message = BitVec::new();
        // (a, b) is 0 if b - <a, s> is closer to 0 than to floor(p/2) mod p otherwise 1
        for (a, b) in ciphertext {
            let dot_product = a
                .iter()
                .zip(secret_key.iter())
                .map(|(&a, &s)| a * s)
                .sum::<Z>();
            if (*b - dot_product).distance_to_zero()
                > (*b - dot_product).distance_to(&Z::new(self.p / 2))
            {
                message.push(Bit::One);
            } else {
                message.push(Bit::Zero);
            }
        }
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gridlock() {
        let mut grid_lock = GridLock::new();
        let secret_key = grid_lock.gen_secret_key();
        let public_key = grid_lock.gen_public_key(&secret_key);
        let message = BitVec::from_bytes(vec![0b10101010]);
        let encrypted = grid_lock.encrypt(&public_key, message.clone());
        let decrypted = grid_lock.decrypt(&secret_key, &encrypted);
        assert_eq!(message.to_bytes(), decrypted.to_bytes());
    }
}
