use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

/// Represents an edge of the graph.
/// The extreme nodes are stored in order by their values.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Edge(u32, u32);

impl Edge {
    pub fn new(a: u32, b: u32) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

/// Compute random `v ∈ {0, 1, ..., n-1}\{a, b}`
/// where `e = (a, b)`.
/// `a` must be different from `b`.
pub fn rand_without(n: u32, Edge(a, b): Edge) -> u32 {
    if a == b {
        panic!("a ({}) equals b ({}) in rand_without", a, b);
    }
    // Sample a random among n-2 nodes
    let mut v = thread_rng().gen_range(0..n - 2);
    if v >= a {
        // skip a
        v += 1;
    }
    if v >= b {
        // skip b
        v += 1;
    }
    v
}

/// Returns the next node to be sampled with probability `p`.
pub fn next_sample(p: f64, start: u32) -> u32 {
    let alpha = thread_rng().gen_range(0f64..=1.);
    start + ((1. - alpha) / (1. - p)).log(1. - p).ceil() as u32 + 1
}

///////////////////////////////////////
//////////////  HASH MAP //////////////
///////////////////////////////////////

#[derive(Copy, Clone)]
pub struct RandomState {
    r: u32,
    r1: u32,
    r2: u32,
}

impl RandomState {
    fn new(r: u32) -> Self {
        let mut gen = thread_rng();
        let r1 = gen.gen_range(1..=r);
        let r2 = gen.gen_range(1..=r);

        Self { r, r1, r2 }
    }
}

pub struct CustomHasher {
    state: RandomState,
    hash: u64,
    count: u8,
}

impl CustomHasher {
    fn new(state: RandomState) -> Self {
        Self {
            state,
            hash: 0,
            count: 0,
        }
    }
}

impl Hasher for CustomHasher {
    fn finish(&self) -> u64 {
        self.hash % (2 * self.state.r as u64)
    }

    fn write_u32(&mut self, i: u32) {
        self.hash += i as u64
            * match self.count {
                0 => self.state.r1,
                1 => self.state.r2,
                _ => unreachable!("This hasher hashes up to 2 objects."),
            } as u64
            % (2 * self.state.r as u64);
        self.count += 1;
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!("This hasher hashes only `u32`.");
    }
}

impl BuildHasher for RandomState {
    type Hasher = CustomHasher;
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new(*self)
    }
}

/// Creates a new HashMap with the custom hash function.
pub fn hash_table<K, V>(r: u32) -> HashMap<K, V, RandomState> {
    HashMap::with_capacity_and_hasher(2 * r as usize, RandomState::new(r))
}
