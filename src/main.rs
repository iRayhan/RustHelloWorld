use crate::test_log_trace::{print_debug, print_error, print_info, print_warn, subscribe_log};

mod test_axum;
mod test_enum;
mod test_error_handling;
mod test_generic;
mod test_linked_list;
mod test_log_trace;
mod test_macro;
mod test_mysql_db;
mod test_ownership;
mod test_struct;
mod test_test;

#[tokio::main]
async fn main() {

    // database
    /*
        init_env();

        let pool = get_pool().await;

        let data = TestTableData {
            id: 0,
            name: Some(String::from("user4")),
        };
        insert(&pool, data).await.unwrap();

        delete(&pool, 6).await.unwrap();

        let get_all = get_all(&pool).await;

        println!("{:?}", get_all);
    */

    // log trace

        subscribe_log();
        print_debug("debug");
        print_warn("warn");
        print_info("info");
        print_error("error");


    // backend
    // axum::serve(get_listener().await, get_notes_route()).await.unwrap();

    /*
        let mut a = sample_struct {
            i: 2
        };
        let mut b = a;
        b = sample_struct {
            i: 3
        };

        let c = &mut a.i;
        *c = 4;

        println!("{:?}, {:?}", a, b);
    */

    // linked_list
    /*
        let a = create_test_linked_list("a", None);

        let b = create_test_linked_list("b", Some(a));
        *b.pointer.borrow_mut() = None;
        // drop(b);

        println!("{:?}", b);
    */

    // test_macro
    /*
    test_macro!(hi);
    let mut a = StructMacro { a: 1 };
    test_macro!(& mut a);

    let mut v: Vec<TestMacro2<String, i32>> = vec![TestMacro2::StringName( String::from("a"))];
    v.push(TestMacro2::IntName(1));
    test_macro!(v);

    let mut v = vec![TestMacro3::AnyType(10.11)];
    test_macro_3!(v);

    let mut v = vec![TestMacro3::AnyType("Gadha")];
    test_macro_3!(v);
    let mut v: Vec<TestMacro3<i32>> = vec![TestMacro3::AnyType( 100)];
    */

    // transfer ownership
    /*
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

    // ref
    let a = Rc::new(RefCell::new(String::from("1")));
    let mut b = a.borrow().to_string();

    a.borrow_mut().push_str("2");
    b.push('3');
    println!("{:?}", a.borrow());
    println!("{:?}", b);
    */

    // struct
    /*
    let father = Father::default();
    println!("{:?}", father);

    let child = Child {
        retries: 2,
        debug: false,
        timeout: 3.0,
    };
    child.debug;
    */

    // enums
    /*
    let message = Message::Write(String::from("hello"), 5);
    println!("{:?}", return_result(5));
    */

    // pattern
    // let (_, x, ..) = (1, "sdfdsaf", 10, 23, 22);

    // error handling
    /*
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
    */

    // generics
    /*
    let a = GenericStruct {
        grade: 101.5,
        error: false,
    };
    println!("{}", get_grade(a).unwrap_or(String::from("unknown")));
    */

    // filter string list
    /*
    let mut name_list = Vec::new();
    name_list.push("a");
    name_list.push("b");
    let filtered_name: Vec<&&str> = name_list.iter().filter(|&name| !name.eq(&"a")).collect();
    println!("{:?}", filtered_name);
    */

    // filter number list
    /*
    let mut number_list = Vec::new();
    number_list.push(1);
    number_list.push(2);
    number_list.push(3);
    let filtered_number: Vec<&i32> = number_list.iter().filter(|&number| number == &3).collect();
    println!("{:?}", filtered_number);
    */

    // create object
    /*
    let person = Person {
        name: "Jon".to_string(),
        age: 25,
        profile: Profile {
            name: "Engineer".to_string(),
            experience: 5,
        },
    };
    // println!("{:?}", person);
    // println!("{:#?}", person);

    // Integer addition
    // println!("1 + 2 = {}", 1u32 + 2);

    // return function
    // println!("1 + 2 = {}", add(1, 2));
    */

    // conditions
    /*
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
    */
}
