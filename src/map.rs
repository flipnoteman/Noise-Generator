#![allow(dead_code)]
#![allow(unused_imports)]
use crate::point::*;

use std::{fmt, path::Path};
use std::fmt::Formatter;
use image::RgbImage;
use rand::{Rng, thread_rng};

const AVG_FACTOR: f32 = 1.0;

/**
 * Map object for manipulation
 **/
pub struct Map{
    pub map: Vec<Vec<f32>>,
    pub apexes: Vec<Point>,
    scalar: f32,
    peak: f32,
    valley: f32
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
            scalar: 0.0,
            peak: 0.0,
            valley: 100.0
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

    /// Gets a value at a position
    pub fn get_value(&mut self, pos: (usize, usize)) -> f32{
        self.map[pos.1][pos.0]
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

            let val = rng.gen_range(0f32..1f32);
            self.set_value(
                point.get(),
                val
            );
            
            if val > self.peak {
                self.peak = val;
            }

            if val < self.valley {
                self.valley = val;
            }
        }
    }

    /// Scales all values in your Map by a given scalar (f32)
    pub fn scale(&mut self, scalar: f32) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.set_value((j, i), self.map[i][j] * scalar);
            }
        }
        self.peak *= scalar;
        self.valley *= scalar;
        self.set_scalar(scalar);
    }

    /// Clears the Map
    pub fn clear(&mut self) {
        self.scale(0.0);
    }

    /// Exports the map to a png
    pub fn export_png(&mut self) {
        println!("Exporting to png...");
        let mut image_buffer = RgbImage::new(self.width() as u32, self.height() as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let m = self.get_value((x as usize, y as usize));
            let rmax = self.peak;
            let rmin = self.valley;
            let tmax = 255.0;
            let tmin = 0.0;
            let mut c = ((m - rmin)/(rmax-rmin))*(tmax-tmin) + tmin;

            if c < 0. {
                c = 0.;
            }

            if c > 255. {
                c = 255.;
            }

            *pixel = image::Rgb([c as u8, c as u8, 255 as u8]);
        }

        image_buffer.save("Map.png").unwrap();
        println!("Exporting Finished!")
    }

    /// Smooths any points in the map in a circular pattern
    pub fn smooth(&mut self, noise: f32, exp: f32) {
        let mut rng = thread_rng();
        let mut avg;
        for y in 0..self.height() {
            'point: for x in 0..self.width() {
                avg = 0.0;
                if self.get_apexes().contains(&Point::new(x, y)) {
                    continue 'point;
                }
                let mut closest = 100.0;
                for apex in self.get_apexes() {
                    let a = apex.get();
                    let c_point = (x, y);
                    let dist = Self::get_distance(c_point, a);
                    if dist < closest {
                        closest = dist;
                    }
                    avg += Self::get_distance(c_point, a) + rng.gen_range(-noise..=noise);
                    avg /= 2.0;
                }
                let val = (closest / avg).powf(exp);
                self.set_value((x, y), val);
            }
        }
    }

    fn get_distance(left: (usize, usize), right: (usize, usize)) -> f32{
        let xs = (right.0 as i32 - left.0 as i32).pow(2);
        let ys = (right.1 as i32 - left.1 as i32).pow(2);
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