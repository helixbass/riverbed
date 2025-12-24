pub struct Range {
    pub start_point: Point,
    pub end_point: Point,
    pub start_byte: u32,
    pub end_byte: u32,
}

impl Range {
    pub fn new(start_point: Point, end_point: Point, start_byte: u32, end_byte: u32) -> Self {
        Self {
            start_point,
            end_point,
            start_byte,
            end_byte,
        }
    }
}

pub struct Point {
    pub row: u32,
    pub column: u32,
}

impl Point {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}
