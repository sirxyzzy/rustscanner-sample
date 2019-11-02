use std::env;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    if !dir.is_dir() {
        println!("{:?} is not a folder I can scan!", dir);
    }

    println!("Scanning folder {:?}...", dir);

    // Some metrics
    let mut file_count = 0;
    let mut skipped_files = 0;
    let mut bytes_read: usize = 0;

    // we are computing the sum of all bytes, in all files
    let mut total: u64 = 0;

    let timer = Instant::now();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            file_count += 1;

            match std::fs::read(entry.path()) {
                Ok(bytes) => {
                    bytes_read += bytes.len();

                    // Why is this loop so slow?
                    for b in bytes {
                        total += b as u64
                    }
                }
                Err(_) => skipped_files += 1,
            }
        }
    }

    let elapsed = timer.elapsed().as_millis();

    println!(
        "Finished in {}ms, found {} files, skipped {}, bytes read {}, checksum {}",
        elapsed, file_count, skipped_files, bytes_read, total
    );

    Ok(())
}
