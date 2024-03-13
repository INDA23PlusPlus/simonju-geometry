use std::cmp::Ordering;

use crate::vector::{cross, line_dist, norm2, side, Side, V, P};

pub fn run() {
    let mut lines = std::io::stdin().lines();
    let mut output = String::new();

    while let Some(Ok(case)) = lines.next() {
        let n = case.parse::<usize>().unwrap();

        if n == 0 { break }

        if n == 1 {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_ascii_whitespace();
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            let y = iter.next().unwrap().parse::<i32>().unwrap();
            output.push_str(&format!("1\n{} {}\n", x, y));
            continue
        }

        let mut points = Vec::with_capacity(n);

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_ascii_whitespace();
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            let y = iter.next().unwrap().parse::<i32>().unwrap();
            points.push(P{ x, y })
        }

        /*************************************************************/

        let mut y_min = points[0].y;
        let mut min = 0;

        for i in 1..n {
            let y = points[i].y;

            if y < y_min || (y == y_min && points[i].x < points[min].x) {
                y_min = points[i].y;
                min = i;
            }
        }

        points.swap(0, min);

        /*************************************************************/

        let bottom = points.remove(0);
        points.sort_unstable_by(|&a, &b| polar_compare(bottom, a, b));

        let mut i = 0;
        while i < points.len() - 1 {
            if cross(points[i] - bottom, points[i + 1] - bottom) == 0 {
                points.remove(i + 1);
            } else {
                i += 1;
            }
        }

        if points.len() == 1 {
            let top = points[0];
            if top.x != bottom.x || top.y != bottom.y {
                output.push_str(&format!("2\n{} {}\n{} {}\n", bottom.x, bottom.y, top.x, top.y));
            } else {
                output.push_str(&format!("1\n{} {}\n", bottom.x, bottom.y));
            }
            continue;
        }

        /*************************************************************/

        let mut stack = Vec::with_capacity(n);

        stack.push(bottom);
        stack.push(points.remove(0));
        stack.push(points.remove(0)); 

        for point in points {
            while cross(stack[stack.len() - 2] - point, stack[stack.len() - 1] - point) < 0 {
                stack.pop();
            }

            stack.push(point)
        }

        if stack.len() == 2 {

        }

        output.push_str(&format!("{}\n", stack.len()));
        for point in &stack {
            output.push_str(&format!("{} {}\n", point.x, point.y));
        }
    }

    println!("{output}");
}

fn polar_compare(bottom: P, a: P, b: P) -> Ordering {
    match cross(a - bottom, b - bottom) {
        left if left > 0 => return Ordering::Less,
        right if right < 0 => return Ordering::Greater,
        _ => (),
    }

    if norm2(a - bottom) > norm2(b - bottom) {
        return Ordering::Less
    }

    return  Ordering::Greater
}