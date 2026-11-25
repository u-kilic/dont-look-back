struct Projectile {
    hitbox:Hitbox,
    sprite:Sprite,
    hits_player:bool,
    creature_effect:fn(), //idk, doesn't seem right
    player_effect:fn(),
    a:Vec2
}