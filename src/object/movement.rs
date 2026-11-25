trait Mover { // creatures/player/projectiles that can move left or right
    fn move_side(side:Side);
}

trait JumpFall { // creatures that don't have this are considered fliers, are unaffected by gravity
    fn jump(); // just the initial launching self off the ground with accelaration
    fn fall(); // update every frame while midair, technically covers rising up too, maybe change name
}