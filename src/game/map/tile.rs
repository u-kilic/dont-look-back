// TODO überprüfen ob Datei nach dem Merge mit mod richtig eingebunden für cargo fmt

pub enum Tile {
    Breakable {
        tile_type: BreakableTile,
        state: BreakableState,
    },
    Unbreakable(UnbreakableTile),
    Empty,
}

pub enum UnbreakableTile {
    Ground,
    Stone,
    //Spikes, //maybe later in seperate categorie Hazard since it behaves different regarding Collision eg
    Bridge, //if you stand under the bridge and jump, you stand on it
}

pub enum BreakableTile {
    Normal,
    MysteryLoot(MysteryLoot),
}

pub enum BreakableState {
    Intact,
    Destroyed,
}

pub enum MysteryLoot {
    Coin,
    Life,
    Potion,
    Note,
}

pub enum TileDeco {
    //in the background, character can pass all of them without collision or effect
    Ruins,
    Stone,
    Plant,
}
