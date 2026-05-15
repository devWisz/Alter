extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::{self, copy, BufReader, Write};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let source_path = args().nth(1).unwrap();
    let target_path = args().nth(2).unwrap();

    let mut input = BufReader::new(File::open(&source_path).unwrap());
    let output = File::create(&target_path).unwrap();
    
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output_file = encoder.finish().unwrap();
    let duration = start.elapsed();

    let input_len = input.get_ref().metadata().unwrap().len();
    let output_len = output_file.metadata().unwrap().len();

    println!("Source len: {:?}", input_len);
    println!("Target len: {:?}", output_len);
    println!("Elapsed:    {:?}", duration);

print!("\n Do you want to  compress one more file?(y/n):");
io::stdout().flush().unwrap();

let mut response = String::new();
io::stdin().read_line(&mut response).unwrap();


if response.trim().to_lowercase()!= "y"{
    return;
}
} 