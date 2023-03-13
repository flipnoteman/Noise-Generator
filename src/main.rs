/** Perlin Noise Generator
 *
 * @name = Sam Smith
 * @date = 3/12/2023
 *
 **/
mod map;
mod point;
use crate::map::*;


const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 15;


fn main() {
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);

    map.seed_rand(10);
    map.scale(100f32);

    println!("{}", map);
}
