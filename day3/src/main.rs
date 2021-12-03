use std::fs;

struct GammaReader {
    num_readings: u64,
    one_counts: [u64; 5],
}

impl GammaReader {
    fn new() -> Self {
        Self {
            num_readings: 0,
            one_counts: [0, 0, 0, 0, 0],
        }
    }

    fn add_reading(&mut self, reading: &str) {
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
                let mask = 1 << (4 - i);
                gamma = gamma | mask;
            }
        }
        gamma
    }

    fn consumption(&self) -> u64 {
        let gamma = self.calc_gamma();
        let mask = (1 << 5) - 1;
        let episilon = !gamma & mask;
        gamma * episilon
    }
}

#[cfg(test)]
mod gamma_reader_tests {
    use crate::GammaReader;

    fn read_test_input(reader: &mut GammaReader) {
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
        let reader = &mut GammaReader::new();
        read_test_input(reader);

        assert_eq!(reader.num_readings, 12);
        assert_eq!(reader.one_counts[0], 7);
        assert_eq!(reader.one_counts[1], 5);
    }

    #[test]
    fn it_calculates_gamma() {
        let reader = &mut GammaReader::new();
        read_test_input(reader);

        assert_eq!(reader.calc_gamma(), 22);
    }

    #[test]
    fn it_calculates_consumption() {
        let reader = &mut GammaReader::new();
        read_test_input(reader);

        assert_eq!(reader.consumption(), 198);
    }
}

fn read_input(path: &String, reader: &mut GammaReader) {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|reading| reader.add_reading(reading.trim()));
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let reader = &mut GammaReader::new();
    read_input(&path, reader);

    println!("{}", reader.num_readings);
    println!("{:?}", reader.one_counts);
    reader.calc_gamma();
}
