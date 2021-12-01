use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let depths = read_depths(&path);
    println!("{}", part1(&depths));
    println!("{}", part2(&depths));
}

fn read_depths(path: &String) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn part1(depths: &Vec<i32>) -> usize {
    depths
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part2(depths: &Vec<i32>) -> usize {
    depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
