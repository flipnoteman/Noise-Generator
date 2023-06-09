#![allow(dead_code)]
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y
        }
    }
}

impl Point {

    pub fn default() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }

    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}