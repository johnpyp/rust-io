// use rayon::prelude::*;
use crossbeam_channel::unbounded;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Instant;
use uuid::Uuid;

static NTHREADS: i32 = 8;
static COUNT: i32 = 10_000_000;
fn main() -> Result<(), Box<dyn Error>> {
    if fs::metadata("uuids.txt").is_ok() {
        fs::remove_file("uuids.txt")?;
    }

    let start = Instant::now();
    let (tx, rx) = unbounded();

    let per_thread = COUNT / NTHREADS;
    println!("Threads: {}", NTHREADS);
    println!("Per thread: {}", per_thread);
    println!("Total: {}", per_thread * NTHREADS);

    for t in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            for _i in 0..per_thread {
                thread_tx.send(Uuid::new_v4().to_string()).unwrap()
            }
            println!("thread {} finished", t);
        });
    }

    let mut res: String = "".to_string();
    let mut count = 0;
    for rec in rx.iter() {
        res.push_str(&rec);
        res.push_str("\n");
        count += 1;
        if count == COUNT {
            break;
        }
    }

    fs::write("uuids.txt", res)?;

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
    Ok(())
}
