/** Perlin Noise Generator
 *
 * @name = Sam Smith
 * @date = 3/12/2023
 *
 **/
mod map;
mod point;
use crate::map::*;


const MAP_WIDTH: i32 = 1080;
const MAP_HEIGHT: i32 = 720;
const SEED_POINTS: usize = 250;


fn main() {
    //let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    //let mut map = map!(MAP_WIDTH, MAP_HEIGHT);
    let mut map = map!(MAP_WIDTH, MAP_HEIGHT);

    println!("Generating and seeding Map: {}x{}...", MAP_WIDTH, MAP_HEIGHT);

    map.seed_rand(SEED_POINTS);

    println!("Smoothing the map out with {} seed points...", SEED_POINTS);
    map.smooth(0.1, 0.6);

    println!("Smoothing Finished.");
    map.scale(255f32);

    map.export_png();

    println!("{}", map);

}
