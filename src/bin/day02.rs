use std::fs;
use std::time::Instant;

fn main() {
    let day_name = "day02";
    let file_path = format!("inputs/{}.txt", day_name);

    let raw_input = fs::read_to_string(&file_path)
        .expect(&format!("Error reading file '{}'. Check your file path!", file_path));

    println!("--- Solving {} ---", day_name);

    // 3. Solve Part 1
    let start = Instant::now();
    let p1_solution = part1(&raw_input);
    let duration = start.elapsed();
    println!("Part 1: {}  (Time: {:?})", p1_solution, duration);

    // 4. Solve Part 2
    let start = Instant::now();
    let p2_solution = part2(&raw_input);
    let duration = start.elapsed();
    println!("Part 2: {}  (Time: {:?})", p2_solution, duration);
}


fn part1(input: &str) -> i64 {
    // Tip: Use input.lines() to iterate over rows
    let mut ans: i64 = 0;
    for range in input.split(",") {
        let range = range.trim();
        if range.is_empty() { continue; }
        
        // we unwrap because split_once returns an Option
        let (left_str, right_str) = range.split_once("-").unwrap();
        let start: i64 = left_str.parse().unwrap();
        let end: i64 = right_str.parse().unwrap();

        for num in start..=end {
            let curr = num.to_string();

            if curr.len() % 2 != 0 {
                continue;
            }
            let mid = curr.len() / 2;

            let (first_half, second_half) = curr.split_at(mid);

            if first_half == second_half {
                ans += num;
            }
        }
    }
    return ans;
}


fn part2(input: &str) -> i64 {
    let mut ans: i64 = 0;
    for range in input.split(",") {
        let range = range.trim();
        if range.is_empty() { continue; }

        let (left_str, right_str) = range.split_once("-").unwrap();
        let start: i64 = left_str.parse().unwrap();
        let end  = right_str.parse().unwrap();

        for num in start..=end {
            let s = num.to_string();
            if is_invalid_pt2(&s) {
                ans += num;
            }
        }

    }
    
    ans
}

fn is_invalid_pt2(s: &str) -> bool {
    let n = s.len();

    // try all possible pattern lengths (k) from [1, n/2]
    for k in 1..=(n / 2) {
        // total length must be divisible by the pattern length
        if n % k == 0 {
            let pattern = &s[0..k];
            let reps = n / k;

            // checking if repeating the pattern recreates the original string
            if pattern.repeat(reps) == s {
                return true;
            }
        }
    }
    false
}

// Unit Tests (Run with `cargo test --bin day01`)
#[cfg(test)]
mod tests {
    use super::*;

    // Paste the "example" input from the AoC website here
    const EXAMPLE_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }
}