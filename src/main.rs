use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::time::Instant;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let res = (0..10_000_000)
        .into_par_iter()
        .map(|_i| Uuid::new_v4().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    fs::write("uuids.txt", res)?;

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
    Ok(())
}
