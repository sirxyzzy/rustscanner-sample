use std::env;
use std::io;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

struct ScanSummary {
    file_count: i32,
    skipped_files: i32,
    bytes_read: usize,
    total: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    if !dir.is_dir() {
        println!("{:?} is not a folder I can scan!", dir);
        return;
    }

    println!("Scanning folder {:?}...", dir);

    let timer = Instant::now();

    let scan_result = scan(&dir);

    let elapsed = timer.elapsed().as_millis();

    match scan_result {
        Ok(s) => println!(
            "Finished in {}ms, found {} files, skipped {}, bytes read {}, checksum {}",
            elapsed, s.file_count, s.skipped_files, s.bytes_read, s.total
        ),
        Err(e) => println!("Scan failed! {}", e),
    }
}

fn scan(dir: &PathBuf) -> io::Result<ScanSummary> {
    // Some metrics
    let mut file_count = 0;
    let mut skipped_files = 0;
    let mut bytes_read: usize = 0;

    // we are computing a checksum over all files
    let mut total: i64 = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            file_count += 1;

            match std::fs::read(entry.path()) {
                Ok(bytes) => {
                    bytes_read += bytes.len();

                    for b in bytes {
                        total += b as i64
                    }
                }
                Err(_) => skipped_files += 1,
            }
        }
    }

    Ok(ScanSummary {
        file_count,
        skipped_files,
        bytes_read,
        total,
    })
}
