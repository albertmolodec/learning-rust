#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)] // Можно проверить значение штата в списке
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter!! {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6("::1".to_string());
    println!("{:?} {:?}", home, loopback);

    let msg = Message::Write(String::from("Hello"));
    msg.call();

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
}
