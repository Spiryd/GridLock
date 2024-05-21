use rand::{distributions::{Distribution, Uniform}, SeedableRng};
use rand_hc::Hc128Rng;

const N: usize = 16;

/// p hould be a prime where p >= 2 and n^2 < p < 2n^2
const P: usize = 263;

/// m = (1 + ε)(n + 1) log p for some arbitrary constant ε > 0
/// e.g. ε = 1
const M: usize = 40;

mod z;
use z::Z;

#[derive(Debug)]
pub struct GrigLock {
    n: usize,
    m: usize,
    p: usize,
    chi: Uniform<Z>
}

impl GrigLock {
    pub fn new() -> GrigLock {
        GrigLock {
            n: N,
            m: M,
            p: P,
            chi: Uniform::new_inclusive(Z::new(0), Z::new(P-1))
        }
    }
    pub fn gen_private_key(&self) -> Vec<Z> {
        self.gen_vector()
    }
    pub fn gen_public_key(&self, s: &Vec<Z>) {
        let a = (0..self.m).map(|_| self.gen_vector()).collect::<Vec<_>>();
        println!("{:?}", a)
    }
    fn gen_vector(&self) -> Vec<Z> {
        let mut rng = Hc128Rng::from_entropy();
        (0..self.n).map(|_| self.chi.sample(&mut rng)).collect()
    }
}

fn main() {
    let griglock = GrigLock::new();
    let s =  griglock.gen_private_key();
    let pk = griglock.gen_public_key(&s);
    println!("{:?}", s);
}
