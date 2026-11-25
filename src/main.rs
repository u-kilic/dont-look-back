mod game;

use game::map::*;


fn main() {
    println!("Hello, world!");
}

type XGame = u64; // to be used with game pixels
type YGame = u64;

type XTile = u64; // to be used with tiles 
type YTile = u64;

enum Side {
    Left,
    Right
}

struct Camera {
    // will camera be only sidescrolling? can it move up/down?
    // it should probably be based on GamePixels in terms of x/y

    x_offset:XGame

}


struct Game {
    pub player:Player,
    pub tile_map:TileMap,
    pub pixel_map:PixelMap,
    pub world:World, //specific gravity, background
    pub creatures:Vec<Creature>
}

struct Image {} //some image type