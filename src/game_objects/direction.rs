#[derive(Copy, Clone, PartialEq)]
pub enum HorizontalDirection {
    Left = -1, Neither = 0, Right = 1
}

#[derive(Copy, Clone, PartialEq)]
pub enum VerticalDirection {
    Up = -1, Neither = 0, Down = 1
}

pub fn vertical_to_i32(direction: &VerticalDirection) -> i32 {
    *direction as i32
}

pub fn horizontal_to_i32(direction: &HorizontalDirection) -> i32 {
    *direction as i32
}
