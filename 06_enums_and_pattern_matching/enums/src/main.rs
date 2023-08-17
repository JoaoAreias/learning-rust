enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8),
}


enum Coin {
    Penny,
    Nickel, 
    Dime,
    Quarter
}



fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);

    // The option type
    let x: u8 = 5;
    let y: Option<u8> = Some(8);
    let sum = x + y.unwrap_or(0);


    // Match expression
    let coin = Coin::Dime;
    let coin_value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    };
}
