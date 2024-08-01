//https://steveklabnik.com/writing/structure-literals-vs-constructors-in-rust
// Interesting read about constructors
[derive(Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    _secret: (),
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x, y: y, _secret: () }
    }
}
