struct PixelMap {
    pub pixels:Box<[GamePixel]>
}

struct GamePixel {
    x:XGame,
    y:YGame,
}