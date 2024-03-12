

pub struct Cursor {
    pub x: i64,
    pub y: i64,
}


pub fn create_cursor() -> Cursor {
    let mut cursor = Cursor {
        x: 0,
        y: 0,
    };
    cursor
}
