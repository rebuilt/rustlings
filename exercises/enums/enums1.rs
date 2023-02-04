// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[derive(Debug)]
enum Response {
    Success,
    Failure,
    Delayed,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Response::Success);
    println!("{:?}", Response::Failure);
}
