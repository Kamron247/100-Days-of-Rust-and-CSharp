enum IpAddrKind{
    v4,
    v6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum enumIpAddr{
    V4(String),
    V6(String),
}

enum betterEnumIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl betterEnumIpAddr{
    fn display(&self){
        println!("Idk how to reference the data!")
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::v4;
    let six  = IpAddrKind::v6;

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let home_v2 = enumIpAddr::V4(String::from("127.0.0.1"));

    let home_v3 = betterEnumIpAddr::V4(127,0,0,1);
    home_v3.display();

    //Option
    let some_num = Some(5);
    let some_char = Some('e');
    let absent_num: Option<i32> = None;

    let lucky_penny = Coin::Penny;
    let state_quarter = Coin::Quarter(UsState::Alaska);
    println!("My pennys value: {}", value_in_cents(lucky_penny));
    println!("My quarters value: {}", value_in_cents(state_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{five:?} + 1 = {six:?}");
    println!("{five:?} + {none:?} = a problem!");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => reroll(),   // used for when i dont want to pass any values
        // _ => (),         // used for when i dont want to pass value and i want to do/return nothing
    }

    let configure_max = Some(3u8);
    if let Some(max) = configure_max {      // similar to match but only executes when matching a single case
        println!("The max is configured to be {max}");
    } // Else is sill available here as the "catch all"

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,       // This option must be covered, otherwise error
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Added one fancy hat!");
}
fn remove_fancy_hat() {
    println!("Removed one fancy hat!");
}
fn move_player(num_spaces: u8) {
    println!("Moving {num_spaces} spaces!")
}