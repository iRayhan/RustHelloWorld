use crate::test_enum::Message;

fn process_message(msg: Message) {
    use Message::*;

    match &msg {
        Quit => println!("Quit"),
        Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Write(text, number) => println!("Text: {}", text),
        ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        _ => println!("other"),
    }

    if let Write(t, _) = msg {
        println!("Write to x: {}", t);
    }
}

#[derive(Debug)]
struct CustomError {
    pub(crate) error_code: i32,
    pub(crate) error_msg: String,
}

pub fn return_custom_result(s: String) -> Result<i32, CustomError> {
    match s.parse() {
        Ok(x) => Ok(x),
        Err(y) => Err(CustomError {
            error_code: 100,
            error_msg: String::from("custom error"),
        }),
    }
}
