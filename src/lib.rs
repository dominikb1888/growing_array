/// DO NOT CHANGE FUNCTION NAME AND INPUT,
/// DO ADD OUTPUT TYPE
pub fn process_growing_array_heap(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let _n = lines[0].parse::<i64>().expect("Parsing Error");
    let mut nums: Vec<i64> = lines[1].split(" ").map(|c| c.parse::<i64>().expect("")).collect();

    format!("{}", increase_array_heap(&mut nums))
}

pub fn increase_array_heap(nums: &mut Vec<i64>) -> i64 {

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


pub fn process_growing_array(input: &str) -> String {
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

pub fn increase_array(nums: &mut [i64]) -> i64 {

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
