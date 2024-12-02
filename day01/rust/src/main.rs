use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let content = read_to_string("../input.txt")?;
    let (left_list, right_list): (Vec<_>, Vec<_>) = content
        .lines()
        .filter(|line| !line.is_empty()) // Skip empty lines
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().expect("Failed to parse number"))
                .collect();
            if nums.len() != 2 {
                panic!("Invalid input format: each line should have exactly 2 numbers");
            }
            (nums[0], nums[1])
        })
        .unzip();

    let calculate_distance = move || {
        let mut sorted_left = left_list;
        let mut sorted_right = right_list;

        sorted_left.sort_unstable(); // sort_unstable is faster than sort
        sorted_right.sort_unstable();

        sorted_left
            .iter()
            .zip(sorted_right.iter())
            .map(|(a, b)| (a - b).abs() as i64) // Using i64 to prevent potential overflow
            .sum::<i64>()
    };

    let result = calculate_distance();
    println!("Total distance: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Create a temporary test file
        std::fs::write(
            "../test_input.txt",
            "97924 12015\n\
             50267 32019\n\
             98415 10716",
        )
        .unwrap();

        // Read and process the test file
        let content = read_to_string("../test_input.txt").unwrap();
        let (left_list, right_list): (Vec<_>, Vec<_>) = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().expect("Failed to parse number"))
                    .collect();
                (nums[0], nums[1])
            })
            .unzip();

        let mut sorted_left = left_list;
        let mut sorted_right = right_list;

        sorted_left.sort_unstable();
        sorted_right.sort_unstable();

        let result = sorted_left
            .iter()
            .zip(sorted_right.iter())
            .map(|(a, b)| (a - b).abs() as i64)
            .sum::<i64>();

        // Clean up the test file
        std::fs::remove_file("../test_input.txt").unwrap();

        // Assert the expected result
        assert_eq!(result, 141590); // Replace with your expected test value
    }
}
