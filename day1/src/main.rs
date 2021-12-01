fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let numbers = get_numbers(path);

    let quickness = numbers.windows(2).fold(0, |count, chunk| {
        if chunk[0] < chunk[1] {
            count + 1
        } else {
            count
        }
    });
    println!("{}", quickness);
}

fn get_numbers(_path: String) -> [i32; 10] {
    let numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    numbers
}
