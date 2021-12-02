use std::fs::File;
use std::io::{BufRead, BufReader};
use triangle_counting::arb_ord::arb_ord;

fn main() {
    let mut reader = BufReader::new(File::open("../datasets/itdk0304").unwrap());

    let mut n = String::from("0");
    reader.read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();

    let lines = reader.lines().map(|l| l.unwrap());
    let edges = lines.map(|l| {
        let (a, b) = l.split_once('\t').unwrap();
        (a.trim().parse().unwrap(), b.trim().parse().unwrap())
    });

    println!("{}", arb_ord(100000, n, edges));
}
