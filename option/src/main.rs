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

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
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
    let i = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", i);
    let n = Some(i);
    let n_1 = plus_one(n);
    let none = plus_one(None);
    println!("{:?}",n_1);
    println!("{:?}",none);
}