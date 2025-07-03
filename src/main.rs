use std::sync::Arc;

fn send() {
    let s = String::from("hello");

    // send the ownership of s to different thread
    std::thread::spawn(move || {
        println!("send {}", s);
    });
}

fn sync() {
    let s = Arc::new(String::from("hello"));
    let s1 = Arc::clone(&s);

    // share the ownership of s with s1 and send to different thread
    std::thread::spawn(move || {
        println!("sync {}", s1);
    });

    // s can still be used in the main thread
    println!("{}", s);
}

fn transfer_ownership(s: String) {
    println!("transfer_ownership {}", s);
}

fn borrow_ownership(s: &String) {
    println!("borrow_ownership {}", s);
}

#[derive(Default, Debug)]
struct Father {
    debug: bool,
    retries: u32,
    timeout: f64,
    child: Child,
}

#[derive(Debug)]
struct Child {
    debug: bool,
    retries: u32,
    timeout: f64,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            debug: false,
            retries: 8,
            timeout: 3.0,
        }
    }
}

impl Child {
    fn new(self: Self) -> Self {
        Self { ..self }
    }
    fn get_debug(&self) -> bool {
        self.debug
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String, i32),
    ChangeColor(u8, u8, u8),
    Other(Child),
    // Text("jbc"),
    JBC,
}

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

fn return_result(value: i32) -> Result<String, String> {
    let a: Result<String, String> = Ok(value.to_string());
    let b: Result<String, String> = Err(value.to_string());
    Err(value.to_string())
}

#[derive(Debug)]
struct CustomError {
    error_code: i32,
    error_msg: String,
}

fn return_custom_result(s: String) -> Result<i32, CustomError> {
    match s.parse() {
        Ok(x) => Ok(x),
        Err(y) => Err(CustomError {
            error_code: 100,
            error_msg: String::from("custom error"),
        }),
    }
}

trait AllowedGrade {
    fn to_int(&self) -> i32;
}
impl AllowedGrade for String {
    fn to_int(&self) -> i32 {
        -1
    }
}
impl AllowedGrade for &str {
    fn to_int(&self) -> i32 {
        -1
    }
}
impl AllowedGrade for i32 {
    fn to_int(&self) -> i32 {
        *self
    } 
}
impl AllowedGrade for f32 {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

trait AllowedError {
    fn from_bool(value: bool) -> Self;
}
impl AllowedError for bool {
    fn from_bool(value: bool) -> Self {
        value
    }
}

struct GenericStruct<T, E>
where
    T: AllowedGrade,
    E: AllowedError,
{
    grade: T,
    error: E,
}

fn get_grade<T, E>(mut grade: GenericStruct<T, E>) -> Option<String>
where
    T: std::fmt::Display + AllowedGrade + std::cmp::PartialOrd,
    E: std::fmt::Display + AllowedError,
{
    match std::any::type_name::<T>() {
        "alloc::string::String" | "&str" => {
            grade.error = E::from_bool(false);
            Some(format!("String grade, error: {} {}", grade.grade, grade.error))
        },
        "i32" => {
            if grade.grade.to_int() > 100 {
                grade.error = E::from_bool(true);
                return Some(format!("Integer grade, error: {} {}", grade.grade, grade.error))
            }
            Some(format!("Integer grade, error: {} {}", grade.grade, grade.error))
        }
        "f32" => {
            if grade.grade.to_int() > 100 {
                grade.error = E::from_bool(true);
                return Some(format!("Float grade, error: {} {}", grade.grade, grade.error))
            }
            Some(format!("Float grade, error: {} {}", grade.grade, grade.error))
        },
        _ => None,
    }
}

#[derive(Debug)]
struct StructMacro {
    a: i32,
}

#[macro_export]
macro_rules! test_macro {
    (hi) => {
       println!("hi"); 
    };
/*    ($struct_macro:expr) => {
        let mut x = $struct_macro;
        x.a += 1;
        println!("test_macro:{:?}", $struct_macro);
    };*/
    ($usr:expr) => {
        let mut x = match $usr[1] {
            User2::IntName(y) => {
                let mut y = y;
                y += 1;
                println!("{}", y);
                y
            },
            _ => {
                println!("test_macro");
                0
            }
        };
        println!("test_macro:{:?}", $usr);
    };
}
struct User <T>{
    name: T
}

#[derive(Debug)]
enum User2{
    StringName(String),
    IntName(u32),
}
fn main() {
    
    // test_macro!(hi);
    let mut a = StructMacro {
        a: 1,
    };
    // test_macro!(& mut a);
    
    let mut v: Vec<User2> = vec![User2::StringName( String::from("a"))];
    v.push(User2::IntName(1));
    test_macro!(v);
    /*// transfer ownership
    let a = String::from("2");
    transfer_ownership(a);

    // borrow ownership with read permission
    let a = String::from("2");
    let b = &a;
    let c = &a;
    borrow_ownership(b);
    borrow_ownership(c);

    // borrow ownership with read and write permission
    let mut a = String::from("2");
    let b = &mut a;
    b.push_str("3");
    borrow_ownership(b);

    // send the ownership to different thread
    send();

    // share the ownership with different thread
    sync();

    // struct
    let father = Father::default();
    println!("{:?}", father);

    let child = Child {
        retries: 2,
        debug: false,
        timeout: 3.0,
    };
    child.debug;

    // enums
    let message = Message::Write(String::from("hello"), 5);
    println!("{:?}", return_result(5));

    // pattern
    let (_, x, ..) = (1, "sdfdsaf", 10, 23, 22);

    // error handling
    let s = String::from("a120a120");
    let i: i32 = match s.parse() {
        Ok(x) => x,
        Err(y) => {
            println!("{:?}", y);
            let mut d = 0;
            for z in s.chars() {
                match z.to_digit(10) {
                    Some(x) => {
                        d *= 10;
                        d += x as i32;
                    }
                    None => {
                        break;
                    }
                };
            }
            if d >= 0 {
                d = -1
            };
            d
        }
    };
    println!("{}", i);

    let result = return_custom_result(s);

    if let Ok(x) = result {
        println!("{:?}", x);
    } else if let Err(y) = result {
        println!("{:?}", y.error_code);
        println!("{:?}", y.error_msg);
    }

    // generics
    let a = GenericStruct { 
        grade: 101.5,
        error: false,
    };
    println!("{}", get_grade(a).unwrap_or(String::from("unknown")));*/

    // *a = String::from("3");

    // println!("{}{}", a, b);
    /*
    // filter string list
    let mut name_list = Vec::new();
    name_list.push("a");
    name_list.push("b");
    let filtered_name: Vec<&&str> = name_list.iter().filter(|&name| !name.eq(&"a")).collect();
    // println!("{:?}", filtered_name);

    // filter number list
    let mut number_list = Vec::new();
    number_list.push(1);
    number_list.push(2);
    number_list.push(3);
    let filtered_number: Vec<&i32> = number_list.iter().filter(|&number| number == &3).collect();
    // println!("{:?}", filtered_number);

    // create object
    let person = Person {
        name: "Jon".to_string(),
        age: 25,
        profile: Profile {
            name: "Engineer".to_string(),
            experience: 5
        }
    };
    // println!("{:?}", person);
    // println!("{:#?}", person);

    // Integer addition
    // println!("1 + 2 = {}", 1u32 + 2);

    // return function
    // println!("1 + 2 = {}", add(1, 2));

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);*/
}

fn some_number() -> Option<u32> {
    Some(42)
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
    profile: Profile,
}

#[derive(Debug)]
struct Profile {
    name: String,
    experience: i8,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
