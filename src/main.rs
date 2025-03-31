use growing_array::{process_growing_array};

/// DO CHANGE CODE BELOW THIS LINE
///
// src/lib.rs or src/main.rs
pub fn process(input: &str) -> String {
    process_growing_array(&input)
}

fn main() {
    // Read from stdin and write to stdout
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let output = process(&input);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Include the dynamically generated test code
    include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
}
