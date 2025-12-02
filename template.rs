use std::fs;
use std::time::Instant;

fn main() {
    let day_name = "dayXX";
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
    // TODO: Write your logic here
    // Tip: Use input.lines() to iterate over rows
    
    0
}


fn part2(input: &str) -> i64 {
    // TODO: Write your logic here
    
    0
}

// Unit Tests (Run with `cargo test --bin day01`)
#[cfg(test)]
mod tests {
    use super::*;

    // Paste the "example" input from the AoC website here
    const EXAMPLE_INPUT: &str = "\
1000
2000
3000
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 0); // Replace '0' with the example answer
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 0); // Replace '0' with the example answer
    }
}