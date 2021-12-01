use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    let quickness = fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |count, chunk| {
            if chunk[0] < chunk[1] {
                count + 1
            } else {
                count
            }
        });
    println!("{}", quickness);
}
