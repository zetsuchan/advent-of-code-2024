use std::collections::HashMap;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let content = read_to_string("../input.txt")?;
    
    // Parse input into two lists
    let (left_list, right_list): (Vec<_>, Vec<_>) = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse().expect("Failed to parse number"))
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    // Create frequency map of right list
    let right_frequencies: HashMap<i64, i64> = right_list
        .iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    // Calculate similarity score
    let similarity_score: i64 = left_list
        .iter()
        .map(|&num| num * right_frequencies.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        std::fs::write("../test_input.txt", test_input).unwrap();

        let content = read_to_string("../test_input.txt").unwrap();
        let (left_list, right_list): (Vec<_>, Vec<_>) = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let nums: Vec<i64> = line
                    .split_whitespace()
                    .map(|n| n.parse().expect("Failed to parse number"))
                    .collect();
                (nums[0], nums[1])
            })
            .unzip();

        let right_frequencies: HashMap<i64, i64> = right_list
            .iter()
            .fold(HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });

        let similarity_score: i64 = left_list
            .iter()
            .map(|&num| num * right_frequencies.get(&num).unwrap_or(&0))
            .sum();

        std::fs::remove_file("../test_input.txt").unwrap();
        assert_eq!(similarity_score, 31);
    }
} 