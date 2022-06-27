// Channel giúp thread hoàn thiện hơn => cho thread giao tiếp &1 chiều 

// Note : Smart pointer 

// 2 loại channel : channel và sync_channel

// Note : crossbeam-channel ( crates.io )

// Trong thực tế kết hợp với enum rất nhiều


// use crossbeam_channel::unbounded;
// use std::thread;

// fn main() {
//     let (sender, receiver) = unbounded();

//     let a = thread::spawn(move || match receiver.recv() {
//         Ok(msg) => println!("{}", msg),
//         Err(e) => println!("{:?}",e)
        
//     });
    
//     sender.send("hello nguyen cao tri");

//     a.join();
// }

//--------------------------------------------------------------------------------------

use crossbeam_channel::unbounded;
use std::thread;

enum Message {
    Printmsg(String),
    Sum(i32,i32),
    Quit,
}
fn main() {
    let (s,r) = unbounded();

    let a = thread::spawn(move || loop {
        match r.recv() {
            Ok(aaa) => match aaa {
                Message::Printmsg(data) => println!("{}", data),
                Message::Sum(a,b ) => println!("{} + {} = {}", a,b,a+b),
                Message::Quit => {
                    println!("quit");
                    break;
                }
            },
            Err(e) => {println!("{:?}",e); 
            break;
        },
        }
    });

    s.send(Message::Printmsg("NGuyen Cao Tri".to_owned()));

    s.send(Message::Sum(10, 5));

    s.send(Message::Quit);

    a.join();
}
