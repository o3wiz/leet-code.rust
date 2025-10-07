fn main() {
    let numbers = vec![1, 3, 2, 1, 2, 3, 4];
    let result: Vec<_> = numbers
        .chunk_by(|r1, r2| r1 < r2)
        .flat_map(|c| 1..=c.len())
        .collect();

    dbg!(&numbers);
    dbg!(&result);
}
