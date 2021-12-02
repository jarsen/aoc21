#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_input(input: String) -> Self {
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
        assert_eq!(
            Command::from_input("forward 0".to_string()),
            Command::Forward(0)
        );
        assert_eq!(
            Command::from_input("forward 1".to_string()),
            Command::Forward(1)
        );
        assert_eq!(Command::from_input("down 5".to_string()), Command::Down(5));
        assert_eq!(Command::from_input("up 7".to_string()), Command::Up(7));
    }

    #[test]
    #[should_panic]
    fn it_panics_on_bad_input() {
        Command::from_input("left 6".to_string());
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

    fn r#move(&self, r#move: Command) -> Submarine {
        match r#move {
            Command::Forward(distance) => Submarine {
                x: self.x + distance,
                y: self.y,
            },
            Command::Down(distance) => Submarine {
                x: self.x,
                y: self.y - distance,
            },
            Command::Up(distance) => Submarine {
                x: self.x,
                y: self.y + distance,
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
            location.r#move(Command::Forward(4))
        );
        assert_eq!(Submarine { x: 0, y: 5 }, location.r#move(Command::Up(5)));
        assert_eq!(Submarine { x: 0, y: -3 }, location.r#move(Command::Down(3)));
    }
}

fn main() {
    println!("Hello, world!");
}
