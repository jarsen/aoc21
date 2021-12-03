fn consumption(gamma: u32) -> u32 {
    let mask = (1 << 5) - 1;
    let episilon = !gamma & mask;
    gamma * episilon
}

#[cfg(test)]
mod consumption_tests {
    use crate::consumption;

    #[test]
    fn calculates_consumption() {
        assert_eq!(consumption(22), 198);
    }
}

fn main() {
    println!("Hello, world!");
}
