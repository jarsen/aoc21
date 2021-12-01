use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    part1(&path);
    part2(&path);
}

fn part1(path: &String) {
    let quickness = fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", quickness);
}

fn part2(path: &String) {
    let quickness = fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", quickness);
}
