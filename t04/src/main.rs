// https://doc.rust-lang.org/book/ch06-00-enums.html

fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6
    // }
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }
    // fn route(ip_kind: IpAddrKind) {}
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // enum IpAddr {
    //     // V4(String),
    //     V4(u8, u8, u8, u8),
    //     V6(String)
    // }
    // // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    
    // struct Ipv4Addr;
    // struct Ipv6Addr;
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr)
    // }

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32)
    // }
    // struct QuitMessage;
    // struct MoveMessage { x: i32, y: i32 }
    // struct WriteMessage(String);
    // struct ChangeColor(i32, i32, i32);
    // impl Message {
    //     fn call(&self) {}
    // }
    // let m = Message::Write(String::from("hello"));
    // m.call();

    // enum Option<T> {
    //     None,
    //     Some(T)
    // }
    // let some_number = Some(5);
    // let some_char = Some('e');
    // let absent_number: Option<i32> = None;
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // Error

    // #[derive(Debug)]
    // enum UsState {
    //     Alabama,
    //     Alaska
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState)
    // }
    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         },
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {state:?}");
    //             25
    //         }
    //     }
    // }

    // fn addone(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1)
    //     }
    // }
    // let five = Some(5);
    // let six = addone(five);
    // let none = addone(None);

    // fn first() {}
    // fn second() {}
    // // fn third(x: u8) {}
    // let dice = 9;
    // match dice {
    //     3 => first(),
    //     7 => second(),
    //     // _ => third(dice)
    //     _ => ()
    // }

    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => ()
    // }

    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to br {max}");
    // }
    
    // #[derive(Debug)]
    // enum UsState {
    //     Alabama,
    //     Alaska
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState)
    // }
    // impl UsState {
    //     fn existed_in(&self, year: u16) -> bool {
    //         match self {
    //             UsState::Alabama => year >= 1819,
    //             UsState::Alaska => year >= 1959,
    //             _ => false
    //         }
    //     }
    // }
    // fn describe_state_quarter(coin: Coin) -> Option<String> {
    //     if let Coin::Quarter(state) = coin {
    //         if state.existed_in(1900) {
    //             Some(format!("{state:?} is pretty old, for America!"))
    //         } else {
    //             Some(format!("{state:?} is relatively new."))
    //         }
    //     } else {
    //         None
    //     }
    // }
    // fn _describe_state_quarter(coin: Coin) -> Option<String> {
    //     // let state = if let Coin::Quarter(state) = coin {
    //     //     state
    //     // } else {
    //     //     return None;
    //     // };
    //     // let Coin::Quarter(state) = coin else { return None; };
    //     // let state = if let Coin::Quarter(state) = coin {
    //     //     state
    //     // } else {
    //     //     return None;
    //     // };
    //     let Coin::Quarter(state) = coin else { return None; };
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     } else {
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // }
    // let mut count = 0;
    // let coin = Coin::Penny;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}"),
    //     _ => count += 1
    // }
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}");
    // } else {
    //     count += 1;
    // }
}