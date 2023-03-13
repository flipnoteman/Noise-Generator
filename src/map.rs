use crate::point::*;

use std::fmt;
use std::fmt::Formatter;
use rand::{Rng, thread_rng};

/**
 * Map object for manipulation
 **/
pub struct Map{
    pub map: Vec<Vec<f32>>,
    pub apexes: Vec<Point>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map {
            map: vec![vec![0.0; width as usize]; height as usize],
            apexes: Vec::<Point>::new()
        }
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn swap(&mut self, pos: (usize, usize), value: f32){
        self.map[pos.1][pos.0] = value;
    }

    pub fn seed_rand(&mut self, seed_points: usize) {
        let mut rng = thread_rng();

        for _ in 0..seed_points {
            let point = Point::new(
                rng.gen_range(0..self.width()),
                rng.gen_range(0..self.height()),
            );
            self.apexes.push(point.clone());

            self.swap(
                point.get(),
                rng.gen_range(0f32..1f32)
            );
        }
    }

    pub fn scale(&mut self, scalar: f32) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.swap((j, i), self.map[i][j] * scalar);
            }
        }
    }

    pub fn
}

/// Implement Display for Map objects
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&*format!("Map: \nwidth={}, height={}", self.width(), self.height()));
        for str in &self.map {
            output.push_str(&*format!("\n\t{:?}", str));
        }
        output.push_str(&*format!("\n\nPoints: {},", self.apexes.len()));
        for pnt in &self.apexes {
            output.push_str(&*format!("\n\t{}", pnt));
        }
        write!(f, "{}", output)
    }
}