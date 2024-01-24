enum IppAddrKind {
    V4(String),
    V6(String),
}

enum IppAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit")
            }
            Message::Move { x, y } => {
                println!("Move to x: {}, y: {}", x, y)
            }
            Message::Write(text) => {
                println!("Text message: {}", text)
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
            }
        }
    }
}

fn main() {
    let _four = IppAddrKind::V4;
    let _six = IppAddrKind::V6;

    let _home = IppAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IppAddrKind::V6(String::from("::1"));

    let _home = IppAddr::V4(127, 0, 0, 1);
    let _loopback = IppAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
