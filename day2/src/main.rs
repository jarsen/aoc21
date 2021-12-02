use std::fs;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_input(input: &str) -> Self {
        let split: Vec<&str> = input.split(' ').collect();
        let direction = split[0];
        let units = split[1].parse().unwrap();
        match direction {
            "forward" => Command::Forward(units),
            "down" => Command::Down(units),
            "up" => Command::Up(units),
            _ => panic!("Unknown direction input"),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn it_parses_inputs() {
        assert_eq!(Command::from_input("forward 0"), Command::Forward(0));
        assert_eq!(Command::from_input("forward 1"), Command::Forward(1));
        assert_eq!(Command::from_input("down 5"), Command::Down(5));
        assert_eq!(Command::from_input("up 7"), Command::Up(7));
    }

    #[test]
    #[should_panic]
    fn it_panics_on_bad_input() {
        Command::from_input("left 6");
    }
}

#[derive(Debug, PartialEq)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn start() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn execute(&self, command: &Command) -> Submarine {
        match command {
            Command::Forward(distance) => Submarine {
                horizontal: self.horizontal + distance,
                depth: self.depth + distance * self.aim,
                aim: self.aim,
            },
            Command::Down(amount) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + amount,
            },
            Command::Up(amount) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - amount,
            },
        }
    }
}

#[cfg(test)]
mod submarine_tests {
    use super::*;

    #[test]
    fn submarine_moves_horizontal() {
        let submarine = Submarine::start().execute(&Command::Forward(5));
        assert_eq!(
            Submarine {
                horizontal: 5,
                depth: 0,
                aim: 0
            },
            submarine
        );
    }

    #[test]
    fn submarine_moves_down() {
        let submarine = Submarine {
            horizontal: 5,
            depth: 0,
            aim: 0,
        }
        .execute(&Command::Down(5))
        .execute(&Command::Forward(8));
        assert_eq!(
            Submarine {
                horizontal: 13,
                depth: 40,
                aim: 5
            },
            submarine
        );
    }

    #[test]
    fn submarine_moves_up() {
        let submarine = Submarine {
            horizontal: 13,
            depth: 40,
            aim: 0,
        }
        .execute(&Command::Up(3))
        .execute(&Command::Forward(8));
        assert_eq!(
            Submarine {
                horizontal: 21,
                depth: 16,
                aim: -3
            },
            submarine
        );
    }
}

fn read_commands(path: &String) -> Vec<Command> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(Command::from_input)
        .collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let commands = read_commands(&path);

    let submarine = commands
        .iter()
        .fold(Submarine::start(), |submarine, command| {
            submarine.execute(command)
        });

    println!("{}", submarine.horizontal * submarine.depth);
}
