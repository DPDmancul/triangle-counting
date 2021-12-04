use crate::utils::{hash_table, next_sample, rand_without, Edge};
use rand::{thread_rng, Rng};
use std::{cell::Cell, rc::Rc};

pub fn arb_ord<I: Iterator<Item = (u32, u32)>>(r: u32, n_nodes: u32, edges: I) -> f64 {
    let mut next = 1; // Next sample
    let mut n_edges = 0; // Number of edges seen so far
    let mut m = 1;
    let mut m_big = r;

    let mut samples = Vec::new();
    let mut missing_edges = hash_table(r);

    let mut rng = thread_rng(); // Random generator

    for (a, b) in edges {
        n_edges += 1;
        print!("{}\r", n_edges);

        let e = Edge::new(a, b);

        if n_edges == next {
            let v = rand_without(n_nodes, e);
            let count = Rc::from(Cell::from(0));

            missing_edges.insert(Edge::new(a, v), Rc::clone(&count));
            missing_edges.insert(Edge::new(b, v), Rc::clone(&count));
            samples.push(count);
            next = next_sample(1. / f64::from(m), n_edges);
        }

        if n_edges == m_big {
            m_big *= 2;
            m *= 2;
            // retain probability is 0.5
            samples.retain(|_| rng.gen());
        }

        if let Some(count) = missing_edges.get(&e) {
            count.set(count.get() + 1);
        }
    }

    print!("\x1b[2K");

    let beta = samples
        .iter()
        .fold(0, |acc, count| if count.get() == 2 { acc + 1 } else { acc });
    f64::from(beta) / samples.len() as f64 * f64::from(n_nodes) * f64::from(n_edges)
}
