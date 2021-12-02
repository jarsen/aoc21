#[derive(Debug, PartialEq)]
enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Move {
    fn from_input(input: String) -> Self {
        let split: Vec<&str> = input.split(' ').collect();
        let direction = split[0];
        let distance = split[1].parse().unwrap();
        match direction {
            "forward" => Move::Forward(distance),
            "down" => Move::Down(distance),
            "up" => Move::Up(distance),
            _ => panic!("Unknown direction input"),
        }
    }
}

#[cfg(test)]
mod move_tests {
    use super::*;

    #[test]
    fn it_parses_inputs() {
        assert_eq!(Move::from_input("forward 0".to_string()), Move::Forward(0));
        assert_eq!(Move::from_input("forward 1".to_string()), Move::Forward(1));
        assert_eq!(Move::from_input("down 5".to_string()), Move::Down(5));
        assert_eq!(Move::from_input("up 7".to_string()), Move::Up(7));
    }

    #[test]
    #[should_panic]
    fn it_panics_on_bad_input() {
        Move::from_input("left 6".to_string());
    }
}

#[derive(Debug, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn r#move(&self, r#move: Move) -> Location {
        match r#move {
            Move::Forward(distance) => Location {
                x: self.x + distance,
                y: self.y,
            },
            Move::Down(distance) => Location {
                x: self.x,
                y: self.y - distance,
            },
            Move::Up(distance) => Location {
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
        let location = Location::origin();
        assert_eq!(Location { x: 4, y: 0 }, location.r#move(Move::Forward(4)));
        assert_eq!(Location { x: 0, y: 5 }, location.r#move(Move::Up(5)));
        assert_eq!(Location { x: 0, y: -3 }, location.r#move(Move::Down(3)));
    }
}

fn main() {
    println!("Hello, world!");
}
