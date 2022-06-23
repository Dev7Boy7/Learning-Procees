
// Impl Trait

// impl Trait tương tự như generic 

fn so_sanh_2soduong(so_thu_nhat: u8, so_thu_hai: u8) -> u8{
    let so_lon_hon = if so_thu_nhat>so_thu_hai {so_thu_nhat} else {so_thu_hai};
    so_lon_hon
}

fn main() {
    println!("So lon hon la : {}", so_sanh_2soduong(100, 50));
}

//-----------------------------------------------------------------------------------------------

// remember PartialOrd - Display 

use std::fmt::Display;

fn so_sanh_2so <T: PartialOrd + Display >(so_thu_nhat: T, so_thu_hai: T) -> T{
    let so_lon_hon = if so_thu_nhat>so_thu_hai {so_thu_nhat} else {so_thu_hai};
    so_lon_hon
}

fn main() {
    println!("So lon hon la : {}", so_sanh_2so(-1007.321, -501.3123));
}

//-----------------------------------------------------------------------------------------------

// Giờ ta tương tự nhìn vào impl Trait . thay vì dùng T , ta có thể mang loại impl Trait 

fn print_it (input: impl Into<String> + std::fmt::Display) {
    println!("You can print mmmmmany things, including {}", input);
}

fn main() {
    let name = "nguyen cao tri";

    let number1 = 909608777;

    let string_name = "Nguyen Cao Tri".to_string();

    print_it(name);

    print_it(string_name);
}

//----------------------------------------------------------------------------------------------------

// Tương tự trong sách , ta sẽ viết 1 đoạn code về sự thay đổi khả năng học tập theo từng thời điểm trong ngày f64, sẽ tăng lên vào đêm ...

enum TimeOfDay {
    morning,
    afternoon,
    evening,
}

fn learning_ability (input: TimeOfDay) -> impl FnMut(f64) -> f64 {
    use TimeOfDay::*;

    match input {
        morning => |x| {
            println!("In the morning , i'm so sleepy. My learning-ability point is {}", x * 0.5);
            x * 0.5
        },
        afternoon => |x| {
            println!("In the afternoon , it's tooo HOT . My learning-ability point is {}", x * 1.0);
            x * 1.0
        },
        evening => |x| {
            println!("In the evening , WOW its perpect . My learning-ability point is {}", x * 999.9);
            x * 999.9
        },
    }
}

fn main() {
    let mut learning_ability_base = 10.0;

    let mut morning = learning_ability(TimeOfDay::morning);

    let mut afternoon = learning_ability(TimeOfDay::afternoon);

    let mut evening = learning_ability(TimeOfDay::evening);

    learning_ability_base = morning(learning_ability_base);

    learning_ability_base = afternoon(learning_ability_base);

    learning_ability_base = evening(learning_ability_base);
}







