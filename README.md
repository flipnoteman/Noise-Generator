# Noise Generator
My attempt at creating noise map generators using "apex" (that's what im calling them) points and relative distances (among other devices).

### Features
- Can generate user defined map size
- Generate random noise between apexes
- Voronoi Texture Gen

### In dev
- Generating different "terrain" types
- Multithread
- Seeding
- Publish as API
- Musgrave Texture Gen
- Perlin Noise Gen


## Voronoi Texture Examples
```rs
use map::*;

fn main() {
    // Declare a new Map object using the builtin macros
    let mut map = map!(MAP_WIDTH, MAP_HEIGHT);
    
    /* Create our "apex" points, as I call them, 
    these are what we base our point calculations 
    on to get that sloping effect */
    map.seed_rand(SEED_POINTS);
  
    /* Apply the Voronoi Smoothing to all the pixels of the map, 
    the first paramater tells the function the range of variance 
    you want in the calculated point (in this case a number 
    from -0.1->0.1 is added), and the second is the exponent 
    to apply to the value calculation (closest / avg).powf(0.6) 
    in this case */
    map.voronoi_smooth(0.1, 0.6);
    
    /* Scale every value by a given scalar, 
    this should ensure no value goes above the given scalar value */
    map.scale(255f32);
    
    // Exports your map to a png, see examples of this below
    map.export_png();

    /* You can also output a numerical representation of 
    this in a matrix form along with the point data for every "apex" and the scalar information */
    println!("{}", map);

}
```
#### Example outputs:
![Map (5)](https://user-images.githubusercontent.com/11511200/224905470-8d2a489a-cc0d-4c5f-9eab-fd8958d54fe3.png)
![Map (1)](https://user-images.githubusercontent.com/11511200/224905539-88b71cf0-8e0f-4854-9ff8-5ebff56a6b46.png)
