pub fn run() {
    let mut lines = std::io::stdin().lines();

    lines.next();

    let mut result: f64 = 0.0;

    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_ascii_whitespace();

        let a = iter.next().unwrap().parse::<f64>().unwrap();
        let b = iter.next().unwrap().parse::<f64>().unwrap();
        let c = iter.next().unwrap().parse::<f64>().unwrap();

        let median_length = (2.0 * (a * a + b * b) - c * c).sqrt() / 2.0;

        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

        result += 2.0 * area / median_length;
    }

    print!("{result}");
}
