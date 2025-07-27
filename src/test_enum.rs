use crate::test_struct::Child;

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String, i32),
    ChangeColor(u8, u8, u8),
    Other(Child),
    // Text("jbc"),
    JBC,
}