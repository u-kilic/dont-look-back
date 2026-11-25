struct SpriteManager { //object that belongs to each creature 
    animations:Vec<Vec<Sprite>> //running is a vector of sprites, idle is another etc. just cycle through the vector
}

struct Sprite { //I boxed image because we might need to manipulate these somehow, idk
    image:Image,
    image_size:ImageSize
}