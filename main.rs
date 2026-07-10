use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut iter = input.split_ascii_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut arr: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut moves: i64 = 0;
    for i in 1..n {
        if arr[i] < arr[i - 1] {
            moves += arr[i - 1] - arr[i];
            arr[i] = arr[i - 1];
        }
    }
    writeln!(out, "{}", moves).unwrap();
}