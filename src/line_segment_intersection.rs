pub fn run() {
    let mut lines = std::io::stdin().lines();
    let mut result = String::new();

    lines.next();

    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap());

        let x1 = iter.next().unwrap();
        let y1 = iter.next().unwrap();
        let x2 = iter.next().unwrap();
        let y2 = iter.next().unwrap();
        let x3 = iter.next().unwrap();
        let y3 = iter.next().unwrap();
        let x4 = iter.next().unwrap();
        let y4 = iter.next().unwrap();

        // Point-point case
        if x1 == x2 && y1 == y2 && x3 == x4 && y3 == y4 {
            if x1 == x3 && y1 == y3 {
                result.push_str(&format!("{x1}.00 {y1}.00\n"));
            } else {
                result.push_str("none\n");
            }

            continue;
        }

        // Point-line case
        if x1 == x2 && y1 == y2 {
            let a = x3 - x1;
            let b = x4 - x1;
            let c = y3 - y1;
            let d = y4 - y1;

            let cross = a * d - b * c;

            let x_min = x3.min(x4);
            let x_max = x3.max(x4);
            let y_min = y3.min(y4);
            let y_max = y3.max(y4);

            if cross == 0 && x_min <= x1 && x1 <= x_max && y_min <= y1 && y1 <= y_max {
                result.push_str(&format!("{x1}.00 {y1}.00\n"));
            } else {
                result.push_str("none\n");
            }

            continue;
        }

        // Line-point case
        if x3 == x4 && y3 == y4 {
            let a = x1 - x3;
            let b = x2 - x3;
            let c = y1 - y3;
            let d = y2 - y3;

            let cross = a * d - b * c;

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            if cross == 0 && x_min <= x3 && x3 <= x_max && y_min <= y3 && y3 <= y_max {
                result.push_str(&format!("{x3}.00 {y3}.00\n"));
            } else {
                result.push_str("none\n");
            }

            continue;
        }

        // Line-line case
        let numerator_a = (x4 - x3) * (y3 - y1) - (y4 - y3) * (x3 - x1);
        let numerator_b = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
        let denominator = (x4 - x3) * (y2 - y1) - (y4 - y3) * (x2 - x1);

        // Collinear subcase
        if numerator_a == 0 && numerator_b == 0 && denominator == 0 {
            if !(x1.min(x2) <= x3.max(x4) && x1.max(x2) >= x3.min(x4) && y1.min(y2) <= y3.max(y4) && y1.max(y2) >= y3.min(y4)) {
                result.push_str("none\n");
                continue;
            }

            let (l1x1, l1y1, l1x2, l1y2) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else if x1 > x2 {
                (x2, y2, x1, y1)
            } else if y1 <= y2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };

            let (l2x1, l2y1, l2x2, l2y2) = if x3 < x4 {
                (x3, y3, x4, y4)
            } else if x3 > x4 {
                (x4, y4, x3, y3)
            } else if y3 <= y4 {
                (x3, y3, x4, y4)
            } else {
                (x4, y4, x3, y3)
            };

            if l1x1 <= l2x1 && l2x2 <= l1x2 && l1y1 <= l2y1 && l2y2 <= l1y2 {
                result.push_str(&format!("{l2x1}.00 {l2y1}.00 {l2x2}.00 {l2y2}.00\n"));
                continue;
            }

            if l2x1 <= l1x1 && l1x2 <= l2x2 && l2y1 <= l1y1 && l1y2 <= l2y2 {
                result.push_str(&format!("{l1x1}.00 {l1y1}.00 {l1x2}.00 {l1y2}.00\n"));
                continue;
            }

            let (l0x1, l0y1, l0x2, l0y2) = if l1x1 < l2x1 {
                (l2x1, l2y1, l1x2, l1y2)
            } else if l1x1 > l2x1 {
                (l1x1, l1y1, l2x2, l2y2)
            } else if l1y1 <= l2y1 {
                (l2x1, l2y1, l1x2, l1y2) 
            } else {
                (l1x1, l1y1, l2x2, l2y2)
            };

            if l0x1 == l0x2 && l0y1 == l0y2 {
                result.push_str(&format!("{l0x1}.00 {l0y1}.00\n"));
                continue;
            }

            result.push_str(&format!("{l0x1}.00 {l0y1}.00 {l0x2}.00 {l0y2}.00\n"));
            continue;
        }

        // Parallel subcase
        if denominator == 0 {
            result.push_str("none\n");
            continue;
        }

        // Cross subcase
        let alpha = numerator_a as f64 / denominator as f64;
        let beta = numerator_b as f64 / denominator as f64;

        if alpha < 0.0 || 1.0 < alpha || beta < 0.0 || 1.0 < beta {
            result.push_str("none\n");
            continue;
        }

        let x0 = x1 as f64 + alpha * (x2 as f64 - x1 as f64);
        let y0 = y1 as f64 + alpha * (y2 as f64 - y1 as f64);

        result.push_str(&format!("{:.2} {:.2}\n", x0, y0));
    }

    print!("{result}");
}