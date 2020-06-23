use rayon::prelude::*;
use std::fs;
use std::time::Instant;
use uuid::Uuid;

fn main() {
    // if fs::metadata("uuids.txt").is_ok() {
    //     fs::remove_file("uuids.txt").unwrap();
    // }
    let start = Instant::now();
    let res = (0..10_000_000)
        .into_par_iter()
        .map(|_i| Uuid::new_v4().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    // fs::write("uuids.txt", res).unwrap();
    println!("{}", res.len());

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}
