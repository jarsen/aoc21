use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let depths = read_depths(&path);
    part1(&depths);
    part2(&depths);
}

fn read_depths(path: &String) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part1(depths: &Vec<i32>) {
    let quickness = depths
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", quickness);
}

fn part2(depths: &Vec<i32>) {
    let quickness = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", quickness);
}
