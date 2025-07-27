
#[derive(Debug)]
pub struct StructMacro {
    pub(crate) a: i32,
}

pub struct TestMacro<T> {
    name: T,
}

#[derive(Debug)]
pub enum TestMacro2<T, E> {
    StringName(T),
    IntName(E),
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
    ($test:expr) => {
        let mut x = match $test[1] {
            TestMacro2::IntName(y) => {
                let mut y = y;
                y += 1;
                println!("{}", y);
                y
            }
            _ => {
                println!("test_macro");
                0
            }
        };
        println!("test_macro:{:?}", $test);
    };
}


#[macro_export]
macro_rules! test_macro_3 {
    ($test3:expr) => {
        for x in $test3 {
            let y = match x.cast_string() {
                "Unknown" => {
                    println!("another type: {:?}", x);
                    x
                }
                _ => {
                    println!("string type: {:?}", x);
                    x
                }
            };
        }
    };
}

#[derive(Debug)]
enum TestMacro3<T> {
    AnyType(T),
}
impl<T> TestMacro3<T> {
    fn cast_string(&self) -> &str {
        match std::any::type_name::<T>() {
            "alloc::string::String" | "&str" => std::any::type_name::<T>(),
            _ => "Unknown",
        }
    }
}
fn test_concurrency() {
    println!("Hello from a thread!")
}

