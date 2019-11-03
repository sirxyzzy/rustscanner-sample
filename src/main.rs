//! An intentionally simple sample application written in Rust
//! 
//! Run from the command line, this checksums the content of all files
//! found under a given folder. If no folder is provided the current
//! folder is scanned

use std::env;
use std::io;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

/// A summary of the results of scanning all the files
struct ScanSummary {
    file_count: i32,
    skipped_files: i32,
    bytes_read: usize,
    checksum: i64,
}

/// Start here
fn main() {
    // gather the args into a list of strings
    let args: Vec<String> = env::args().collect();

    // process args, if provided
    let dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    // make sure we really have a folder to start at
    if !dir.is_dir() {
        // print and bail if not
        println!("{:?} is not a folder I can scan!", dir);
        return;
    }

    println!("Scanning folder {:?}...", dir);

    // Time the scan
    let timer = Instant::now();

    let scan_result = scan(&dir);

    let elapsed = timer.elapsed().as_millis();

    // Print summary of result
    match scan_result {
        Ok(s) => println!(
            "Finished in {}ms, found {} files, skipped {}, bytes read {}, checksum {}",
            elapsed, s.file_count, s.skipped_files, s.bytes_read, s.checksum
        ),
        Err(e) => println!("Scan failed! {}", e),
    }
}

/// Checksum all files found recursively under the given path
fn scan(dir: &PathBuf) -> io::Result<ScanSummary> {
    let mut file_count = 0;
    let mut skipped_files = 0;
    let mut bytes_read: usize = 0;
    let mut checksum: i64 = 0;

    // We use an external module, WalkDir, to walk the file tree
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            file_count += 1;

            match std::fs::read(entry.path()) {
                Ok(bytes) => {
                    bytes_read += bytes.len();

                    for b in bytes {
                        checksum += b as i64
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
        checksum,
    })
}
