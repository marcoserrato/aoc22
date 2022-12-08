use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max :i32 = 0;
    let mut vec = Vec::new();

    if let Ok(lines) = read_lines("inputs/input_1_1.txt") {
        let mut current : i32 = 0;
        for line in lines {
            if let Ok(calories_str) = line {
                match calories_str.parse::<i32>() {
                    Ok(n) => {
                        current = current + n
                    },
                    Err(_) => {
                        vec.push(current);
                        if current > max {
                            max = current;
                        }
                        current = 0;
                    }
                }
            }
        }
    }
    vec.sort();
    println!("Top 3: {:?}", vec.iter().rev().take(3).sum::<i32>());
    println!("Top Calories: {}", max);
}


// Taken from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
