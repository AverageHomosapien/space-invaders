//https://steveklabnik.com/writing/structure-literals-vs-constructors-in-rust
// Interesting read about constructors
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    _secret: (),
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x: x, y: y, _secret: () }
    }
}
