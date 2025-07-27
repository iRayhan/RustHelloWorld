use std::sync::Arc;

pub fn transfer_ownership(s: String) {
    println!("transfer_ownership {}", s);
}

pub fn borrow_ownership(s: &String) {
    println!("borrow_ownership {}", s);
}

pub fn send() {
    let s = String::from("hello");

    // send the ownership of s to different thread
    std::thread::spawn(move || {
        println!("send {}", s);
    });
}

pub fn sync() {
    let s = Arc::new(String::from("hello"));
    let s1 = Arc::clone(&s);

    // share the ownership of s with s1 and send to different thread
    std::thread::spawn(move || {
        println!("sync {}", s1);
    });

    // s can still be used in the main thread
    println!("{}", s);
}
