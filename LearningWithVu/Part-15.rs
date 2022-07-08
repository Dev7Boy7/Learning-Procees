
// OOP in Rust , Trait Objects and Box 

// Box kết hợp với Trait objects rất hữu ích 

// Ôn tập về stack và heap 

// Trait Object được lưu ở trên heap

// Tất cả các dữ liệu được biên dịch trên Dynamic Dispatch đều phải đánh đổi về hiệu năng , tốc độ bộ nhớ 

// Ôn tập về Collection 

//--------------------------------------------------------------------------------------------------------

trait Clicky {
    fn click(&self) -> String;
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) -> String {
        "Keyboard input".to_string()
    }
}

struct Mouse;

impl Clicky for Mouse {
    fn click(&self) -> String {
        "Mouse click".to_string()
    }
}

fn main() {
    // let x = Keyboard;
    // x.click();

    // let y = Mouse;
    // y.click();

    // let x: &dyn Clicky = &Keyboard; // Cách này thường không sử dụng vì liên quan đến tham chiếu 

    let x: Box<dyn Clicky> = Box::new(Keyboard);

    let y: Box<dyn Clicky> = Box::new(Mouse);


    let clickers = vec![x,y];

    for u in clickers {
        println!("{}",u.click());
    }
}

/*      Về sau sẽ sử  dụng nhiều dưới dạng phức tạp hơn 
     
        type a = Rc<RefCell<Box<Keyboard>>>;
*/ 

