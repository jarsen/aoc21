struct PowerReading {
    gamma: u32,
    epsilon: u32,
}

impl PowerReading {
    fn consumption(&self) -> u32 {
        self.gamma * self.epsilon
    }
}

#[cfg(test)]
mod power_reading_tests {
    use crate::PowerReading;

    #[test]
    fn calculates_consumption() {
        assert_eq!(
            PowerReading {
                gamma: 22,
                epsilon: 9
            }
            .consumption(),
            198
        );
    }
}

fn main() {
    println!("Hello, world!");
}
