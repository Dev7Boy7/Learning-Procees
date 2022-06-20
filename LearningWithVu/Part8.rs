<!-- https://www.youtube.com/watch?v=e6LZKxU5mjI&list=PLFnEYduGTiXE2ejxmzTIraP2feI-pmeuw&index=9 -->

// Rust không phải là ngôn ngữ lập trình hướng đối tượng

// Trait giúp tạo ra 1 nhóm chức năng có thể áp dụng cho nhiều Struct cùng đặc điểm.

struct Data {
    number1: u32,
    number2: i32,
    str1: String,
    optional : Option<i32>,
}
struct Data2 {
    number1: u32,
    number2: u32,
    str1: String,
    optional : Option<i32>,
}

impl Data {
    fn new() -> Self {
        Data {
            number1: 15,
            number2: 25,
            str1: "SomeString 2".to_string(),
            optional: None,
        }
    }
}

trait Transform {
    fn revert(&self) -> String {
        String::from("No string")
    }
    fn output_revert(&self) {
        println!("{}", self.revert());
    }
}

impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}

impl Transform for Data2 {
    fn revert(&self) -> String {
        (self.number1 + self.number2).to_string()
    }
}
fn main() {
    // let a = Data {
    //     number1: 10,
    //     number2: 20, 
    //     str1: String::from("Some String"),
    //     optional: Some(10),
    // };

    let a =Data::new();

    a.output_revert();

    // println!("{}", a.revert());

    let b = Data2 {
        number1: 10,
        number2: 30,
        str1: "nguyencaotri".to_string(),
        optional: None,
    };

    b.output_revert();

}



