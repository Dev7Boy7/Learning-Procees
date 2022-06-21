// https://www.youtube.com/watch?v=lbo9RU0pxDE&list=PLFnEYduGTiXE2ejxmzTIraP2feI-pmeuw&index=6

// Tạo một file băng cargo new --lib

//-----------------------------------------------------------------------------------------------

mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn take_payment() {}
    }
}

fn call_order() {}

mod back_house {
    fn cook_order(){}
    fn fix_order(){
        super::call_order(); //super lùi lại 1 lv // có thể dùng nhiều super::super
        cook_order();
    }
}

fn eat_at_restaurant() {
    self::front_house::hosting::add_to_waitlist();

    front_house::hosting::add_to_waitlist();
}

//----------------------------------------------------------------------------------------
// use rand::RngCore;
// use rand::Rng;
// use rand::SeedableRng;

use rand::{Rng, RngCore, SeedableRng};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }
    
    pub enum salad {    // đối với enum khi pun thì mọi thứ bên trong được pub 
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast{
            toast: String::from("fish"),
            fruit: String::from("banana"),
            }
        }
    }
}

mod front_house;
// sau đó tạo 1 file mới nằm trong SRC tên front_house.rs
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }


fn eat_res() {
    front_house::hosting::add_to_waitlist();
}

fn eat_at_restaurant () {
    let mut order = back_house::Breakfast::monday("fish");

    order.toast = String::from("chicken");

    let order2 = back_house::Breakfast {
        toast: String::from("wheat"),
        fruit: String::from("apple"),
    };

    let order3 = back_house::salad::Salad;

    let order4 = back_house::salad::Soup;
}
