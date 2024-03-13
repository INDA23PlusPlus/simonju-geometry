pub fn run() {
    let mut lines = std::io::stdin().lines();
    let mut result = String::new();

    loop {
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        if n == 0 { break }

        let mut polygon = vec![];

        for _ in 0..n {
            polygon.push(lines.next().unwrap().unwrap().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }

        let m = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        for p in 0..m {
            let point = lines.next().unwrap().unwrap().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let x = point[0];
            let y = point[1];

            let mut left_inside = false;
            let mut right_inside = false;

            let mut point1 = &polygon[0];

            for i in 1..n+1 {
                let point2 = &polygon[i % n];

                let x1 = point1[0];
                let y1 = point1[1];

                let x2 = point2[0];
                let y2 = point2[1];

                if (y < y1) != (y < y2) {
                    if (x - x1) * (y2 - y1).abs() > (x2 - x1) * (y - y1) {
                        left_inside = !left_inside;
                    }

                    if (x - x1) * (y2 - y1).abs() < (x2 - x1) * (y - y1) {
                        right_inside = !right_inside;
                    }
                } else if y == y1 && y == y2 && ((x1 < x && x < x2) || (x2 < x && x < x1) ) {
                    left_inside = !left_inside;
                } else if y == y1 && y != y2 && x == x1 {
                    left_inside = !left_inside;
                }

                point1 = point2;
            }

            if left_inside && right_inside {
                result.push_str("in\n");
            } else if !left_inside && !right_inside {
                result.push_str("out\n");
            } else {
                result.push_str("on\n");
            }
        }
    }

    print!("{result}");
}

// _TOP_
// out
// on
// out

// _MIDDLE_
// out
// on
// in
// on
// out

// _BOTTOM_
// out
// on
// on
// on
// out