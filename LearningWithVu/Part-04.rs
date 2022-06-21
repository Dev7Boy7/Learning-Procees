//https://www.youtube.com/watch?v=daOoyk9d43I&list=PLFnEYduGTiXE2ejxmzTIraP2feI-pmeuw&index=5

// enum - Option

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6
}

#[derive(Debug)]
struct _IpAddress {
    kind : IpAddressKind,
    address : String,
}
fn main(){
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let localhost = _IpAddress {
        kind: IpAddressKind::V4,
        address: "127.0.0.1".to_string(),
    };
    println!("Local host = {:?}", localhost);
}

fn route ( ipkind: IpAddressKind) {}

// -----------------------------------------------------------------------

#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

fn main(){
    let localhost = IpAddressKind::V4("127.0.0.1".to_string());

    println!("Local host = {:?}", localhost);
}

fn route ( ipkind: IpAddressKind) {}

//-------------------------------------------------------

#[derive(Debug)]
enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

// #[derive(Debug)]
// struct _IpAddress {
//     kind : IpAddressKind,
//     address : String,
// }
fn main(){
    let localhost = IpAddressKind::V4(127,0,0,1);

    println!("Local host = {:?}", localhost);
}

//--------------------------------------

#[derive(Debug)]
enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
struct _IpAddress {
    kind : IpAddressKind,
    address : String,
}

impl _IpAddress {
    fn some_function() {
        println!("Blockchain Development");
    }
}
fn main(){
    let localhost = IpAddressKind::V4(127,0,0,1);

    _IpAddress::some_function();

    println!("Local host = {:?}", localhost);
}

//---------------------------------------------------

// Option Enum : Quan trong 
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }
}

fn main() {
    // let number = 5;
    // let number1 = Some(5);
    // let string1 = Some("String");
    // let nonumber: Option<i32> = None;

    let x = 5;
    let y = Some(5);
    // let sum = x + y; //error

    let sum = x + y.unwrap_or(0);  //unwrap_or()
}

//--------------------------------------------------------

Match Option
#[derive(Debug)]
enum Coin {
    Solana,
    Ether,
    Near,
    Bitcoin(Balance),
}

#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

fn decimals(coin: Coin) -> u8 {
    use Coin::*;
    match coin {
        Solana => {
            println!("Solana");
            1
        }
        Ether => 10,
        Near => 20,
        Bitcoin(aaa) => {
            println!("I am a {:?}", aaa);
            100
        }
    }
}
fn main() {
    decimals(Coin::Solana);
    decimals(Coin::Bitcoin(Balance::Shark));
}

// -----------------------------------------------------

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some (x + 1),
        _ => None,
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);

    println!("{:?}", six);

    let nn = plus_one(None);

    println!("{:?}", nn);
}

//--------------------------------------------------------

fn main() {
    let _value = Some(5);
    // match _value {
    //     Some(5)=> println!("bang 5"),
    //     _ => (),
    // }
    if let Some(5) = _value { println!("bang 5")};
}

