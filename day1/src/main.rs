fn main() {
    let numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let quickness = numbers.windows(2).fold(0, |count, chunk| {
        if chunk[0] < chunk[1] {
            count + 1
        } else {
            count
        }
    });
    println!("{}", quickness);
}
