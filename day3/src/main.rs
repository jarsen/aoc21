use std::fs;

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

    fn calc_gamma(&self) {
        for count in self.one_counts {
            if count > self.num_readings / 2 {
                print!("1");
            } else {
                print!("0");
            }
        }
    }
}

#[cfg(test)]
mod gamma_reader_tests {
    use crate::GammaReader;

    #[test]
    fn it_reads() {
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
        let reader = &mut GammaReader::new();
        for reading in input.lines() {
            reader.add_reading(reading);
        }
        assert_eq!(reader.num_readings, 12);
        assert_eq!(reader.one_counts[0], 7);
        assert_eq!(reader.one_counts[1], 5);
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
