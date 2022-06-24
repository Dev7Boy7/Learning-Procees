
// Thread : Luồng -- Nơi giúp thực thi lệnh 

// Trong Rust khi ta tạo 1 main thì mặc định -> main thread ( 1 luồng ) 

// Multi-thread giúp thay vì chạy line-by-line theo mặc định sẽ chạy cùng lúc => Tận dụng hiệu năng của máy tính => Nâng cao hiệu suất 

// Lưu ý khi tạo thread cần chú ý vấn đề giao tiếp giữa các luồng => Thread có thể là con dao 2 lưỡi => cần lên kế hoạch sử dụng 

// 

// use std::thread::JoinHandle;
// use std::time::Duration;
// use std::thread;

// fn main() {
//     let itarations = 10;

//     let a: JoinHandle<usize> = thread::spawn(move || {
//         thread::sleep(Duration::from_secs(5));
//         50
//     });
//     println!("waiting for data");
//     match a.join() {
//         Ok(value) => println!("Value: {}", value),
//         Err(e) => println!("{:?}", e),
//     }

//     // let b: JoinHandle<usize> = thread::spawn(move || {
//     //     for i in 1..=10 {
//     //         println!("{}", i)
//     //     }
//     // });

//     // a.join().unwrap();

//     // b.join().unwrap();
// }

// Bài tập :

// Yêu cầu : Tạo ra function chạy các thread ( tối thiểu 3 ) => in ra 1 câu nào đó

use std::thread::spawn;
use std::time::Duration;

fn hello() -> &'static str {
    std::thread::sleep(Duration::from_secs(3));
    "Hello"
    
}

fn my_name() -> &'static str {
    std::thread::sleep(Duration::from_secs(3));
    "My name is"
    
}

fn tri_cao() -> &'static str{
    std::thread::sleep(Duration::from_secs(3));
    "Nguyen Cao Tri !!"
    
}

fn main() {
    let aa = spawn(move || hello());
    
    let bb = spawn(move || my_name());
    
    let cc = spawn(move || tri_cao());
    
    let aaa = aa.join().unwrap();

    let bbb = bb.join().unwrap();

    let ccc = cc.join().unwrap();

    println!("{}", aaa);
    
    println!("{}", bbb);
    
    println!("{}", ccc);
}
