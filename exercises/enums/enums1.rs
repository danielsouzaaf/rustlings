// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Quit")));
    println!("{:?}", Message::Echo(String::from("Echo")));
    println!("{:?}", Message::Move(String::from("Move")));
    println!("{:?}", Message::ChangeColor(String::from("ChangeColor")));
}
