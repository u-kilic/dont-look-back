struct TileMap {
    pub tiles:Box<[Option<Tile>]>,
    pub size:u64
}

struct Tile {
    breakable:bool,
    phase_up:bool, //does the block let you jump up through it
    image:Image
}