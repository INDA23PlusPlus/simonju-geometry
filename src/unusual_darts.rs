use std::{ops::Div, str::FromStr};

pub fn run() {
    let mut lines = std::io::stdin().lines();
    let mut result = String::new();

    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut points = [V::default(); 7];

        for i in 0..7 {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_ascii_whitespace();

            let x = iter.next().unwrap().parse::<f64>().unwrap();
            let y = iter.next().unwrap().parse::<f64>().unwrap();

            points[i] = V { x, y, i: (i + 1) };
        }

        let p = lines.next().unwrap().unwrap().parse::<f64>().unwrap();

        let mut least = String::from_str("9").unwrap();

        let q = if simple(&points) { 
            area(&points).abs().div(4.0).powi(3) 
        } else { 
            0.0 
        };

        if (q - p).abs() <= 1e-5 {
            let mut s = String::new();
            for point in points {
                s.push_str(&format!("{} ", point.i));
            }

            if s < least {
                least = s;
            }
        }

        let permutations = Permutations::new(&mut points);

        for permutation in permutations {
            let q = if simple(&permutation) { 
                area(&permutation).abs().div(4.0).powi(3) 
            } else { 
                0.0 
            };

            if (q - p).abs() <= 1e-5 {
                let mut s = String::new();
                for point in permutation {
                    s.push_str(&format!("{} ", point.i));
                }

                if s < least {
                    least = s;
                }
            }
        }

        result.push_str(&least);
        result.push('\n');
    }

    print!("{result}");
}

/* STUFF */

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct V { x: f64, y: f64, i: usize }

#[inline(always)]
fn sub(a: V, b: V) -> V {
    V { x: a.x - b.x, y: a.y - b.y, i: 0 }
}

#[inline(always)]
fn cross(a: V, b: V) -> f64 {
    a.x * b.y - a.y * b.x
}

#[inline(always)]
fn area(points: &[V; 7]) -> f64 {
    let mut area = 0.0;
    let mut j = points.len() - 1;

    for i in 0..points.len() {
        area += cross(points[i], points[j]);
        j = i;
    }

    area / 2.0
}

#[inline(always)]
fn left(a: V, b: V, p: V) -> bool {
    cross(sub(b, a), sub(p, a)) > 0.0
}

#[inline(always)]
fn intersects(a: V, b: V, c: V, d: V) -> bool {
    left(a, b, c) != left(a, b, d) && left(c, d, a) != left(c, d, b)
}

#[inline(always)]
fn simple(points: &[V; 7]) -> bool {
    for i in 0..7 {
        let a = points[i];
        let b = points[(i + 1) % 7];

        'a: for j in i + 2..7 {
            let c = points[j];
            let d = points[(j + 1) % 7];

            if a.x == d.x && a.y == d.y { continue 'a }
            if b.x == c.x && b.y == c.y { continue 'a }

            if intersects(a, b, c, d) { return false }
        }
    }

    true
}

/**************************************************/
struct Permutations<'a> { points: &'a mut [V; 7], cycles: Vec<usize>, i: usize }

impl<'a> Permutations<'a> {
    fn new(points: &'a mut [V; 7]) -> Self {
        let cycles = vec![0; points.len()];
        let i = 0;

        Self { points, cycles, i }
    }

    #[inline(always)]
    fn heap_permutation(&mut self) -> Option<[V; 7]> {
        while self.i < self.points.len() {
            if self.cycles[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.points.swap(0, self.i);
                } else {
                    self.points.swap(self.cycles[self.i], self.i);
                }
                self.cycles[self.i] += 1;
                self.i = 0;
                return Some(self.points.clone());
            } else {
                self.cycles[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}

impl<'a> Iterator for Permutations<'a> {
    type Item = [V; 7];

    fn next(&mut self) -> Option<Self::Item> {
        self.heap_permutation()
    }
}