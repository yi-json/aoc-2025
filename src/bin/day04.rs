use std::fs;
use std::time::Instant;

fn main() {
    let day_name = "day04";
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

const DIRS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];


fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let rows = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    let mut ans: i64 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] != '@' { continue; }

            let mut count = 0;
            for (dr, dc) in DIRS.iter() {
                let nr = r + dr;
                let nc = c + dc;

                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    if grid[nr as usize][nc as usize] == '@' {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                ans += 1;
            }
        }
    }

    ans
}


fn part2(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    let mut ans = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] != '@' {continue;}

                let mut count = 0;
                for (dr, dc) in DIRS.iter() {
                    let nr = r + dr;
                    let nc = c + dc;

                    if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                        if grid[nr as usize][nc as usize] == '@' {
                            count += 1;
                        }
                    }
                }

                if count < 4 {
                    to_remove.push((r, c));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r as usize][c as usize] = '.';
            ans += 1;
        }
    }

    
    ans
}

// Unit Tests (Run with `cargo test --bin day01`)
#[cfg(test)]
mod tests {
    use super::*;

    // Paste the "example" input from the AoC website here
    const EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }
}