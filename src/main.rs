use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = "quotes.txt";
    let output_path = "quotes_formatted.txt";

    // Read the input file line by line
    let file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    // Write selected lines to the output file
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    for (i, line) in reader.lines().enumerate() {
        if let Ok(mut line) = line {
            // every fifth line is a quote
            if i % 5 == 4 {
                // Check if the line starts with an uppercase letter
                if let Some(first_char) = line.chars().next() {
                    if !first_char.is_uppercase() {
                        line = format!("...{}", line);
                    }
                }

                // Check if the line ends with a letter or a number
                if let Some(last_char) = line.chars().rev().next() {
                    if last_char.is_alphanumeric() {
                        line = format!("{}...", line);
                    }
                }

                writeln!(output_file, "{}", line).expect("Failed to write to output file");
            }
        }
    }

    println!("Selected lines written to {}", output_path);
}
