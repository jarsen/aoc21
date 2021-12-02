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
        let distance = split[1].parse().unwrap();
        match direction {
            "forward" => Command::Forward(distance),
            "down" => Command::Down(distance),
            "up" => Command::Up(distance),
            _ => panic!("Unknown direction input"),
        }
    }
}

#[cfg(test)]
mod move_tests {
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
    x: i32,
    y: i32,
}

impl Submarine {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn execute(&self, command: &Command) -> Submarine {
        match command {
            Command::Forward(distance) => Submarine {
                x: self.x + distance,
                y: self.y,
            },
            Command::Down(distance) => Submarine {
                x: self.x,
                y: self.y + distance,
            },
            Command::Up(distance) => Submarine {
                x: self.x,
                y: self.y - distance,
            },
        }
    }
}

mod location_tests {
    use super::*;

    #[test]
    fn can_move_location() {
        let location = Submarine::origin();
        assert_eq!(
            Submarine { x: 4, y: 0 },
            location.execute(&Command::Forward(4))
        );
        assert_eq!(Submarine { x: 0, y: -5 }, location.execute(&Command::Up(5)));
        assert_eq!(
            Submarine { x: 0, y: 3 },
            location.execute(&Command::Down(3))
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
}
