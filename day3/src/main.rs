use std::fs;

struct EnergyReport {
    bit_length: usize,
    num_readings: u64,
    one_counts: Vec<u64>,
}

impl EnergyReport {
    fn new() -> Self {
        Self {
            bit_length: 0,
            num_readings: 0,
            one_counts: Vec::new(),
        }
    }

    fn add_reading(&mut self, reading: &str) {
        if self.bit_length == 0 {
            self.bit_length = reading.len();
            self.one_counts = vec![0; self.bit_length]
        }

        self.num_readings += 1;

        for (i, value) in reading.chars().enumerate() {
            if value == '1' {
                self.one_counts[i] += 1;
            }
        }
    }

    fn calc_gamma(&self) -> u64 {
        let mut gamma = 0;
        for (i, count) in self.one_counts.iter().enumerate() {
            if *count > self.num_readings / 2 {
                let mask = 1 << (self.bit_length - 1 - i);
                gamma = gamma | mask;
            }
        }
        gamma
    }

    fn calc_consumption(&self) -> u64 {
        let gamma = self.calc_gamma();
        let mask = (1 << self.bit_length) - 1;
        let episilon = !gamma & mask;
        gamma * episilon
    }
}

#[cfg(test)]
mod gamma_reader_tests {
    use crate::EnergyReport;

    fn read_test_input(reader: &mut EnergyReport) {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        for reading in input.lines() {
            reader.add_reading(reading);
        }
    }

    #[test]
    fn it_reads() {
        let reader = &mut EnergyReport::new();
        read_test_input(reader);

        assert_eq!(reader.num_readings, 12);
        assert_eq!(reader.one_counts[0], 7);
        assert_eq!(reader.one_counts[1], 5);
    }

    #[test]
    fn it_calculates_gamma() {
        let reader = &mut EnergyReport::new();
        read_test_input(reader);

        assert_eq!(reader.calc_gamma(), 22);
    }

    #[test]
    fn it_calculates_consumption() {
        let reader = &mut EnergyReport::new();
        read_test_input(reader);

        assert_eq!(reader.calc_consumption(), 198);
    }
}

fn read_input(path: &String, reader: &mut EnergyReport) {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|reading| reader.add_reading(reading.trim()));
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let reader = &mut EnergyReport::new();
    read_input(&path, reader);

    println!("Consumption: {}", reader.calc_consumption());
}
