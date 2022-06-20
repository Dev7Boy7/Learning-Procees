// https://www.youtube.com/watch?v=JxA2Y0G80sg&list=PLFnEYduGTiXE2ejxmzTIraP2feI-pmeuw&index=11

// Error handle & result
// result và Option được sử dụng rất nhiều 


// fn main() {
//     panic!("nguyencaotri");
// }
// => thread 'main' panicked at 'nguyencaotri', src\main.rs:6:5


// enum Result <T, E> {
//     Ok(T),
//     Err(E),
// }
// T đại diện cho 1 kiểu dữ liệu bất kì
// Vd:
// fn try_login() -> Result<i32, String> {
//     Ok(1),
//     Err(String::from("denied"))
// }


//Result giúp hạn chế sử dụng if/else


// Authentucation - xác minh
// Authorization - quyền truy cập


struct Emloyee {
    position: Position,
    status: Status,
}

enum Position {
    CEO,
    CTO,
    IT,
    Manager,
    Marketer,
}

enum Status {
    Active,
    Denied,
}

fn try_access(emloyee: &Emloyee) -> Result<(), String> {
    match emloyee.status {
        Status::Denied => return Err("Denied".to_string()),
        _ => (),
    }

    match emloyee.position {
        Position::CEO => Ok(()),
        Position::CTO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_string()),
    }
}

fn print_access( emloyee: &Emloyee) -> Result<(), String> {
    let access = try_access(emloyee)?;
    println!("access");
    Ok(())
}

fn main() {
    let manager = Emloyee { position: Position::Manager, status: Status::Active };

    // let abc = try_access(&manager);

    // if abc.is_ok() {
    //     println!("Access");
    // }
    // println!("{:?}", abc);

    let tienle = Emloyee {position: Position::IT, status: Status::Denied};

    print_access(&tienle);
    
    print_access(&manager);

}

// Tìm hiểu thêm về cách thức hoạt động của ? operator
