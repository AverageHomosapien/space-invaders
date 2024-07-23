#[derive(Copy, Clone)]
pub enum Direction {
    Left = -1, Right = 1
}

pub fn f(direction: &Direction) -> i32 {
    *direction as i32
}
