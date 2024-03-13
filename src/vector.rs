use std::{fmt::Debug, ops::{Add, Mul, Sub}, str::FromStr};

pub type P = V;

#[derive(Default, Clone, Copy)]
pub struct V {
    pub x: i32,
    pub y: i32,
}

impl Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v({},{})", &self.x, &self.y)
    }
}

impl Add for V {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for V {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<i32> for V {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<V> for i32 {
    type Output = V;
    fn mul(self, rhs: V) -> V {
        V { x: self * rhs.x, y: self * rhs.y }
    }
}

#[inline(always)]
pub fn dot(a: V, b: V) -> i32 {
    a.x * b.x + a.y * b.y
}

#[inline(always)]
pub fn cross(a: V, b: V) -> i32 {
    a.x * b.y - a.y * b.x
}

#[inline(always)]
pub fn norm(a: V) -> f32 {
    (norm2(a) as f32).sqrt()
}

#[inline(always)]
pub fn norm2(a: V) -> i32 {
    a.x.pow(2) + a.y.pow(2)
}

#[inline(always)]
pub fn line_dist(a: P, b: P, p: P) -> f32 {
    cross(b - a, p - a) as f32 / norm(b - a)
}

#[inline(always)]
pub fn side(a: P, b: P, p: P) -> Side {
    let cross = cross(b - a, p - a);

    match cross {
        g if g > 0 => Side::Left,
        l if l < 0 => Side::Right,
        _ => Side::Colinear,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Left,
    Colinear,
    Right,
}

impl FromStr for V {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.split_whitespace();
        
        Ok(Self {
            x: if let Some(x) = l.next() {
                match x.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => return Err(())
                }
            } else { 
                return Err(())
            },
            y: if let Some(x) = l.next() {
                match x.parse::<i32>() {
                    Ok(x) => x, Err(_) => return Err(()) 
                } 
            } else { 
                return Err(()) 
            }
        })
    }
}