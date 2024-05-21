use rand::distributions::uniform::{SampleUniform, UniformInt, UniformSampler};

use crate::P;

#[derive(Debug, Clone, Copy)]
pub struct Z(usize);

impl std::fmt::Display for Z {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Z {
    pub fn new(value: usize) -> Z {
        Z (value % P)
    }
}

impl std::ops::Add for Z {
    type Output = Z;

    fn add(self, other: Z) -> Z {
        Z ((self.0 + other.0) % P)
    }
}

impl std::ops::Sub for Z {
    type Output = Z;

    fn sub(self, other: Z) -> Z {
        Z ((self.0 + P - other.0) % P)
    }
}

impl std::ops::Mul for Z {
    type Output = Z;

    fn mul(self, other: Z) -> Z {
        Z ((self.0 * other.0) % P)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UmiformZ(UniformInt<usize>);

impl UniformSampler for UmiformZ {
    type X = Z;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized {
        
        UmiformZ(UniformInt::<usize>::new(low.borrow().0, high.borrow().0))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized {
            UmiformZ(UniformInt::<usize>::new_inclusive(low.borrow().0, high.borrow().0))
    }

    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Z(self.0.sample(rng))
    }
}

impl SampleUniform for Z {
    type Sampler = UmiformZ;
}
