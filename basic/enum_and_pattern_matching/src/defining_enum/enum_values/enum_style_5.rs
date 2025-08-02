// types embedded in its variants

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// if we write enum Message with struct, could be like this
// which is seperate between variant.
#[allow(dead_code)]
struct QuitMessage; // unit struct

#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct WriteMessage(String); // tuple struct

#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        println!("this is function that implement to enum Message.")
    }
}

pub fn enum_style_5() {
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("{:?}", m);
}
