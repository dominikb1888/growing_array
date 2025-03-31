/// DO NOT CHANGE FUNCTION NAME AND INPUT,
/// DO ADD OUTPUT TYPE
fn process_growing_array(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines[0].parse::<usize>().expect("Parsing Error");

    // Ensure we don't exceed the array size
    assert!(n <= 200000, "Input exceeds maximum allowed size");

    let mut nums = [0i64; 200000]; // Allocate fixed-size array on heap

    for (i, num) in lines[1].split_whitespace().enumerate().take(n) {
        nums[i] = num.parse::<i64>().expect("Failed to parse integer");
    }

    format!("{}", increase_array(&mut nums[..n])) // Pass only the relevant slice
}

fn increase_array(nums: &mut [i64]) -> i64 {

    let mut rounds = 0;

    // Traverse through the array to calculate the necessary rounds
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            // Calculate how much needs to be added to nums[i] to make it >= nums[i - 1]
            let deficit = nums[i - 1] - nums[i];
            rounds += deficit;
            nums[i] = nums[i - 1];  // Simulate fixing the current element
        }
    }

    rounds
}

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
