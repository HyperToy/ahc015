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
        fs: [Usize1; N * N],
    }
    for i in 0..N * N {
        input! {
            from &mut source,
            p: Usize1,
        }
        println!(
            "{}",
            if i == N * N - 1 {
                'F'
            } else {
                match (fs[i], fs[i + 1]) {
                    (_, 0) => 'F',
                    (0, _) => 'B',
                    (_, 1) => 'R',
                    (_, 2) => 'L',
                    (_, _) => 'F',
                }
            }
        );
    }
}
