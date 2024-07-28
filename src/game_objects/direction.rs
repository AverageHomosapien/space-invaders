#[derive(Copy, Clone)]
pub enum HorizontalDirection {
    Left = -1, Neither = 0, Right = 1
}

pub enum VerticalDirection {
    Up = -1, Neither = 0, Down = 1
}

pub fn f(direction: &VerticalDirection) -> i32 {
    *direction as i32
}

pub fn f(direction: &HorizontalDirection) -> i32 {
    *direction as i32
}
