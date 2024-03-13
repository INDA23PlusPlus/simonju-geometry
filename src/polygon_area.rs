pub fn run() {
    let mut lines = std::io::stdin().lines();
    let mut result = String::new();

    loop {
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        if n == 0 { break }

        let mut points = vec![];

        for _ in 0..n {
            points.push(lines.next().unwrap().unwrap().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }

        let mut area = 0;
        let mut j = n - 1;

        for i in 0..n {
            area += points[i][0] * points[j][1] - points[i][1] * points[j][0];
            j = i;
        }

        if area < 0 {
            result.push_str("C");
            area *= -1;
        }

        let area = area as f64 / 2.0;

        result.push_str(&format!("CW {:.1}\n", area));
    }

    print!("{result}");
}
