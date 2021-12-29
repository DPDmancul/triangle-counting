use crate::utils::{hash_table, next_sample, Edge};
use rand::{thread_rng, Rng};
use std::{cell::Cell, rc::Rc};

/// Data of a missing edge
struct Missing {
    index: u8,
    counts: Rc<Cell<[u32; 3]>>,
}

pub fn incidence<I: Iterator<Item = (u32, u32)>>(r: u32, edges: I) -> f64 {
    let mut next = 1; // Next sample
    let mut n_paths = 0; // Number of paths of length 2 seen so far
    let mut m = 1;
    let mut m_big = r;
    let mut u = None; // Current source node

    let mut missing_edges = hash_table(r);
    let mut adjacent = Vec::new();

    let mut rng = thread_rng(); // Random generator

    for (a, b) in edges {
        if n_paths < next {
            if Some(a) != u {
                adjacent.clear();
                u = Some(a);
            }
            adjacent.push(b);

            if let Some(Missing { index, .. }) = missing_edges.get_mut(&Edge::new(a, b)) {
                if *index < 2 {
                    *index += 1;
                }
            }

            n_paths += adjacent.len() as u32 - 1;
        }
        while n_paths >= next {
            let w = adjacent[adjacent.len() + next as usize - n_paths as usize - 2];
            let edge = Edge::new(w, b);

            if let Some(Missing { index, counts }) = missing_edges.get(&edge) {
                let mut tmp = counts.get();
                tmp[usize::from(*index)] += 1;
                counts.set(tmp);
            } else {
                missing_edges.insert(
                    edge,
                    Missing {
                        index: 0,
                        counts: Rc::from(Cell::from([1, 0, 0])),
                    },
                );
            }
            next = next_sample(1. / f64::from(m), next);
        }
        while next >= m_big {
            m_big *= 2;
            m *= 2;
            missing_edges.retain(|_, Missing { counts, .. }| {
                let mut delete = false;
                let c @ [c1, c2, c3] = counts.get().map(|c| {
                    let mut deleted = 0;
                    for _ in 0..c {
                        if rng.gen() {
                            deleted += 1;
                            delete = true;
                        }
                    }
                    c - deleted
                });
                counts.set(c);
                !(/*delete &&*/c1 == 0 && c2 == 0 && c3 == 0)
            });
        }
    }

    let beta = missing_edges
        .iter()
        .fold(0, |mut acc, (_, Missing { index, counts })| {
            let [c1, c2, _] = counts.get();
            if *index > 0 {
                acc += 2 * c1;
            }
            if *index == 2 {
                acc += c2;
            }
            acc
        });
    f64::from(n_paths) / 3. * f64::from(beta) / missing_edges.len() as f64
}
