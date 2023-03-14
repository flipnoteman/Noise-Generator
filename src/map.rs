#![allow(dead_code)]
#![allow(unused_imports)]
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
    scalar: f32
}

/// Macro definition for Map creation, you can supply one, or two numbers (comma delimited) to specify the size of your map
#[macro_export] macro_rules! map {
    ($width:literal, $height:literal) => {
        Map::new($width, $height)
    };

    ($w:literal) => {
        Map::new($w, $w)
    };

    ($width:expr, $height:expr) => {
        Map::new($width, $height)
    };

    ($w:expr) => {
        Map::new($w, $w)
    };
}

/*
    Helper methods for the Map class
    */
impl Map {

    /// Defines a new Map with a width and height
    pub fn new(width: i32, height: i32) -> Map {
        Map {
            map: vec![vec![0.0; width as usize]; height as usize],
            apexes: Vec::<Point>::new(),
            scalar: 0.0
        }
    }

    /// Returns the width of the Map
    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    /// Returns the Height of the Map
    pub fn height(&self) -> usize {
        self.map.len()
    }

    /// Sets a value at a position to a new value (f32)
    pub fn set_value(&mut self, pos: (usize, usize), value: f32){
        self.map[pos.1][pos.0] = value;
    }

    /// Places [seed_points] random values at random positions in your map
    pub fn seed_rand(&mut self, seed_points: usize) {
        let mut rng = thread_rng();

        for _ in 0..seed_points {
            let point = Point::new(
                rng.gen_range(0..self.width()),
                rng.gen_range(0..self.height()),
            );
            self.apexes.push(point.clone());

            self.set_value(
                point.get(),
                rng.gen_range(0f32..1f32)
            );
        }
    }

    /// Scales all values in your Map by a given scalar (f32)
    pub fn scale(&mut self, scalar: f32) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.set_value((j, i), self.map[i][j] * scalar);
            }
        }
        self.set_scalar(scalar);
    }

    /// Clears the Map
    pub fn clear(&mut self) {
        self.scale(0.0);
        self.set_scalar(0.0);
    }

    pub fn smooth(&mut self) {
        let temp = Point::new(0, 0);
        for y in 0..self.height() {
            for x in 0..self.width() {
                for apex in self.get_apexes() {
                    println!("{}", Self::get_distance((x, y), apex.get()));
                }
            }
        }
    }

    fn get_distance(left: (usize, usize), right: (usize, usize)) -> f32{
        let xs = right.0 as i32 - left.0 as i32;
        let ys = right.1 as i32 - left.1 as i32;
        let xmy = xs as f32 + ys as f32;
        let sqxmy = (xmy as f32).sqrt();
        sqxmy
    }

    fn set_scalar(&mut self, scalar: f32) {
        self.scalar = scalar;
    }

    fn get_apexes(&self) -> Vec<Point> {
        self.apexes.clone()
    }
}

/// Implement Display for Map objects
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&*format!("Map: \nwidth={}, height={}", self.width(), self.height()));
        for str in &self.map {
            output.push_str(&*format!("\n\t{:?}", str));
        }
        output.push_str(&*format!("\n\nApexes: {},", self.apexes.len()));
        for pnt in &self.apexes {
            output.push_str(&*format!("\n\t{}", pnt));
        }
        output.push_str(&*format!("\n\nScalar: {}", self.scalar));
        write!(f, "{}", output)
    }
}