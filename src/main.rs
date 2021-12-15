use std::fs::File;
use std::io::{BufRead, BufReader};
use triangle_counting::arb_ord::arb_ord;
use triangle_counting::incidence::incidence;

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

    println!("{}", arb_ord(10000, n, edges));

    // To create the incidence list use:
    // ```bash
    //  FILE=itdk0304; cat <(head $FILE --lines=1) <(cat <(tail $FILE --lines=+2) <(tail $FILE --lines=+2 | sed 's/\(.*\)\t\(.*\)/\2\t\1/') | sort -n) > $FILE.incidence
    // ```

    let mut reader = BufReader::new(File::open("../datasets/itdk0304.incidence").unwrap());

    let mut n = String::from("0");
    reader.read_line(&mut n).unwrap();

    let lines = reader.lines().map(|l| l.unwrap());
    let edges = lines.map(|l| {
        let (a, b) = l.split_once('\t').unwrap();
        (a.trim().parse().unwrap(), b.trim().parse().unwrap())
    });

    println!("{}", incidence(10000, edges));
}
