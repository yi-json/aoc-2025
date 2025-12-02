use std::fs;
use std::time::Instant;
// use itertools::Itertools;

fn main() {
    let day_name = "day01";
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

// Logic for Part 1
fn part1(input: &str) -> i64 {
    // starting pt is at 50
    let mut curr = 50;
    let mut count: i64 = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }

        let dir = &line[0..1];
        let dist_str = &line[1..];
        let dist: i64 = dist_str.parse().expect("Should be a number");
        
        // if curr = 5 and inp = L10
        // GOAL: curr = 95
        match dir {
            "L" =>  curr = (curr - dist).rem_euclid(100),
            "R" => curr = (curr + dist).rem_euclid(100),
            _ => panic!("Unknown direction: {}", dir),
        }
        
        if curr == 0 { count += 1;}

    }
    return count;
}

// Logic for Part 2
fn part2(input: &str) -> i64 {
    let mut abs_curr = 50;
    let mut count: i64 = 0;

    for line  in input.lines() {
        if line.is_empty() { continue; }

        let dir = &line[0..1];
        let dist_str = &line[1..];
        let dist: i64 = dist_str.parse().expect("Should be a number");
        
        match dir {
            "L" =>  {
                // count how many multiples of 100 we cross in [end, start)
                // we offset by -1 bc leaving a 0 downwards doesn't count as hitting it
                let next = abs_curr - dist;
                count += (abs_curr - 1).div_euclid(100) - (next - 1).div_euclid(100);
                abs_curr = next;
            },
            "R" => {
                // count how many multiples of 100 we cross in (start, end]
                // we use div_euclid for math floor division
                let next: i64 = abs_curr + dist;
                count += next.div_euclid(100) - abs_curr.div_euclid(100);
                abs_curr = next;
            },
            _ => panic!("Unknown direction: {}", dir),
        }
    }
    return count;
}

// Unit Tests (Run with `cargo test --bin day01`)
#[cfg(test)]
mod tests {
    use super::*;

    // Paste the "example" input from the AoC website here
    const EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }
}