use std::fs;
use std::time::Instant;

fn main() {
    let day_name = "day03";
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


fn part1(input: &str) -> u32 {
    let mut ans: u32 = 0;
    for line in input.lines() {
        // convert to bytes for O(1) random access
        let digits = line.as_bytes();
        let n = digits.len();

        let mut curr = 0;
        
        for i in 0..n {
            for j in (i+1)..n {
                let tens = (digits[i] - b'0') as u32;
                let ones = (digits[j] - b'0') as u32;
                let val = tens * 10 + ones;
                curr = curr.max(val);
            }
        }

        ans += curr;
    }
    ans
}


fn part2(input: &str) -> i64 {
    let mut total_joltage: i64 = 0;

    for line in input.lines() {
        let digits = line.as_bytes();
        let n = digits.len();
        
        // We need to pick exactly 12 digits
        let needed_count = 12;
        
        if n < needed_count { continue; } // Safety check

        let mut current_idx = 0;
        let mut current_val: i64 = 0;

        // We build the number by picking one digit at a time, 12 times.
        for i in 0..needed_count {
            // How many digits MUST we leave for the future steps?
            // If we are picking the 1st digit (i=0), we need to leave 11.
            let digits_to_reserve = needed_count - 1 - i;
            
            // The search window ends so that we save enough space.
            // Example: If len is 15 and we need to reserve 11, we can't search past index 3.
            let search_limit = n - digits_to_reserve;
            
            let mut best_digit = 0;
            let mut best_idx_in_window = current_idx;

            // Scan the valid window for the largest digit
            for j in current_idx..search_limit {
                let d = digits[j];
                if d > best_digit {
                    best_digit = d;
                    best_idx_in_window = j;
                    
                    // Optimization: 9 is the max possible, so we can stop searching early
                    // if we find one. This speeds it up significantly.
                    if best_digit == b'9' {
                        break;
                    }
                }
            }

            // Append the digit to our number
            // (byte - b'0') converts ASCII '5' to integer 5
            let numeric_val = (best_digit - b'0') as i64;
            current_val = current_val * 10 + numeric_val;

            // Move our start pointer to just after the one we picked
            current_idx = best_idx_in_window + 1;
        }

        total_joltage += current_val;
    }

    total_joltage
}

// Unit Tests (Run with `cargo test --bin day01`)
#[cfg(test)]
mod tests {
    use super::*;

    // Paste the "example" input from the AoC website here
    const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 3121910778619);
    }
}