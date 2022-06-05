use core::panic;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path;

/// The function containing the script functionality
pub fn run_script(
    input_path_str: &String,
    output_path_str: &String,
    deliminator_list_str: &String,
) {
    // Create paths given files
    let input_path = Path::new(&input_path_str);
    let output_path = Path::new(&output_path_str);

    // Get display values for the paths
    let input_path_display = input_path.display();

    // Open the input file
    let input_file = match File::open(&input_path) {
        Err(why) => panic!("Couldn't open {}: {}", input_path_display, why),
        Ok(file) => file,
    };

    // Read the input list file into a vector of Strings
    let input_reader = BufReader::new(input_file);
    let input_lines = input_reader.lines().map(|l| l.unwrap()).collect();

    // Process the string
    let output_lines = process_input_lines(&input_lines, deliminator_list_str);

    // Write hte output lines
    match write_output(output_path, output_lines) {
        Err(e) => panic!("Could not open output file: {}", e),
        _ => (),
    };
}

fn process_input_lines(input_lines: &Vec<String>, deliminator_list_str: &String) -> Vec<String> {
    input_lines
        .into_iter()
        .map(|l| l.to_string())
        .filter(|x| !x.contains(deliminator_list_str))
        .collect()
}

fn write_output(output_path: &Path, output_lines: Vec<String>) -> std::io::Result<()> {
    // Define settings for the output file
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(output_path)?;

    // Open a buffered writer to the output file
    let mut file = BufWriter::new(file);

    // Write to the file
    for line in output_lines {
        file.write(line.as_bytes())?;
        file.write("\n".as_bytes())?;
    }

    // Ensure all writes are made
    file.flush()?;

    // Signify the write has completed successfully
    Ok(())
}
