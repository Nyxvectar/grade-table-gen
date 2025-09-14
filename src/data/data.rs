/**
 * Author:  Raye Lattice
 * Repo:    table-gen
 * Created: 09/14/2025
 */
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn data_reader() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("/home/vespr/Downloads/data");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut all_data: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("#[!table-gen(data)]") || trimmed_line == "-eof-" {
            continue;
        }
        if !trimmed_line.is_empty() {
            let line_without_semicolon = trimmed_line.strip_suffix(';').unwrap_or(trimmed_line);
            let numbers: Vec<i32> = line_without_semicolon
                .split(',')
                .map(|s| s.trim().parse().expect("failure: decoding figures"))
                .collect();
            all_data.push(numbers);
        }
    }

    println!("Successfully read {} lines of data:", all_data.len());
    for (i, row) in all_data.iter().enumerate() {
        println!("Line {}: {:?}", i + 1, row);
    }
    Ok(())
}
