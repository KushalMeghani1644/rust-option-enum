fn main() {
    //Rust code for option enums
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;
    //println!("Some number: {:?}", some_number);
    let x = 5;
    let y: Option<i8> = None;
    //let add = x + y;
    let add = x + y.unwrap_or(0); //The unwrap_or method allows us to add a optional variable to a integer variable
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    };
    if let Some(3) = some_value {
        println!("Three");
    }
}

/* Rust code for using the match expression */

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
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
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
