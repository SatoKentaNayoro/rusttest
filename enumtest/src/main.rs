// #[derive(Debug)]
// enum IpAddrKind {
//     v4,
//     v6,
// }
//
//
// fn route(ip_type: &IpAddrKind) {
//     println!("got_type: {:#?}",ip_type)
// }


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    aaa,
    bbb,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("got Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

#[derive(Debug)]
pub enum Option<T> {
    Some(T),
    None,
}

pub fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        Option::None=> Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}

fn main() {
    // let four = IpAddrKind::v4;
    // let six = IpAddrKind::v6;
    // route(&four);
    // route(&IpAddrKind::v6);
    // println!("4:{:#?}", four);
    // println!("6:{:#?}", six);
    let value = value_in_cents(Coin::Quarter(UsState::aaa));
    println!("{}", value);

    let five: Option<i32> = Option::Some(5);
    let six = plus_one(&five);
    let none = plus_one(&Option::None);
    println!("five:{:?}",five);
    println!("six:{:?}",six);
    println!("none:{:?}",none);

    let some_u8_value = Option::Some(3);
    match some_u8_value {
        Option::Some(4) => println!("666"),
        _ => println!("888"),
    }
    if let Option::Some(3) = some_u8_value {
        println!("666")
    }else {
        println!("888")
    }

}
