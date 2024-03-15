#![allow(non_snake_case)]

use proconio::{input, marker::Usize1, source::line::LineSource};
use std::io::{stdin, BufReader};

pub const N: usize = 10;
pub const M: usize = 3;

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        fs: [Usize1; N],
    }
    for _ in 0..N * N {
        input! {
            from &mut source,
            p: Usize1,
        }
        println!("{}", 'F');
    }
}
