
// Rc & Arc - Reference Counter 

// Rc hiểu đơn giản là chia sẻ chủ sở hữu 

// Arc là Atomic Rc - An toàn khi sử dụng với Multithread 

use std::rc::Rc;

#[derive(Debug)]
struct Car {
    number: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Car>,
}
fn main() {
    let car = Rc::new(Car { number: "47A - 63550".to_string(), 
    });

    let left_door = Door {
        vehicle: Rc::clone(&car),
    };

    let right_door = Door { 
        vehicle: Rc::clone(&car),
    };

    
    println!("Left door: {:?}", left_door.vehicle);
    drop(left_door);
    println!("Right door: {:?}", right_door.vehicle);
}


