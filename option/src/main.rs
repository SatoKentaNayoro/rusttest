use crate::Coin::Penny;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


// #[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
    }
}


fn main() {
    let i = value_in_cents(Coin::Quarter(UsState::Alabama));
    print!("{}",i)
}