//https://dhghomon.github.io/easy_rust/Chapter_42.html

//lets you return a &str if you don't need a String, and a String if you need it

use std::borrow::Cow;

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}

fn main() {
    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!("{} went in. The Cow is borrowed with this message: {}", number, message),
            Cow::Owned(message) => println!("{} went in. The Cow is owned with this message: {}", number, message),
        }
    }
}


// Cần tìm hiểu :

// .into() ?
// format! => Tạo ra String 
// fn main() {

//     let a = "nguyencaotri";

//     let b = format!("{}", a);

//     let c: String = a.into();

//     println!("{}", a);
//     println!("{}", b);
//     println!("{}", c);


// Cow<'static, str> 


// Reddit 
// My main use case for `Cow` is avoiding (string) copies. Consider

fn how_many_potions(num: usize) -> std::borrow::Cow<'static, str> {
    match num {
        0 => "out of potions".into(),
        1 => "last potion".into(),
        _ => format!("{} potions remaining", num).into()
    }
}
// Essentially, we want to return a String here, but there are cases in which we can do better because we actually have a &'static str containing the data, so we don't need to allocate and copy a brand new String for this purpose.

// This is exactly what Cow is for.

