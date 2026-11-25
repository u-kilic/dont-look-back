//idek if this will be used depending on crates

struct Button {
    text:String,
    width:u64, // these should be relative to the screen size, but that means we need to come up with an abstract screen size to map the real screen size over to.
    length:u64,
    x:u64,
    y:u64,
}