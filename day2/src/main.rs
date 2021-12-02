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
mod tests {
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

fn main() {
    println!("Hello, world!");
}
