use std::{cmp::min, iter::Sum};

use rand::distributions::uniform::{SampleUniform, UniformInt, UniformSampler};

use crate::P;

#[derive(Debug, Clone, Copy, Eq)]
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
    pub fn distance_to(&self, other: &Z) -> usize {
        match self.cmp(other) {
            std::cmp::Ordering::Less => min(other.0 - self.0, self.0 + P - other.0),
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => min(self.0 - other.0, other.0 + P - self.0),
        }
    }
    pub fn distance_to_zero(&self) -> usize {
        if self.0 > P / 2 {
            P - self.0
        } else {
            self.0
        }
    }
}

impl PartialEq for Z {
    fn eq(&self, other: &Z) -> bool {
        self.0 == other.0
    }   
}

impl PartialOrd for Z {
    fn partial_cmp(&self, other: &Z) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Z {
    fn cmp(&self, other: &Z) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::ops::Add for Z {
    type Output = Z;

    fn add(self, other: Z) -> Z {
        Z ((self.0 + other.0) % P)
    }
}

impl std::ops::AddAssign for Z {
    fn add_assign(&mut self, other: Z) {
        *self = Z ((self.0 + other.0) % P);
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

impl Sum for Z {
    fn sum<I: Iterator<Item = Z>>(iter: I) -> Z {
        iter.fold(Z::new(0), |acc, x| acc + x)
    }
}
