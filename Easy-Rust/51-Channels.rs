
// Channels

// Easy way to use many threads -> one place 

// std::sync::mpsc -- mpsc "mutiple producer, single consumer"

// pub fn channel<T>() -> (Sender<T>, Receiver<T>)

// use std::sync::mpsc::channel;

// fn main() {
//     let (sender, receiver) = channel();

//     sender.send("tricao").unwrap();
//     println!("{}", receiver.recv().unwrap()); // recv = receive, not " rec v "
// }

// a channel like Arc - can clone and send into other threads . 

// use std::sync::mpsc:: channel;
// use std::thread::spawn;

// fn main() {
//     let (sender, receiver) = channel();

//     let sender_clone = sender.clone();

//     spawn(move || { sender.send("Send Nguyen Cao Tri fdsssssssssssssssssssssssssssssssssssssssdsfdsfsdfsdfsd").unwrap(); });

//     spawn(move || { sender_clone.send("And Send").unwrap(); });

//     println!("{:?}", receiver.recv().unwrap());
// }

// tuỳ vào cái nào xong trước ta có thể nhận được trước 

// PS C:\Users\trinc\hoctap> cargo run
// Finished dev [unoptimized + debuginfo] target(s) in 0.01s
// Running `target\debug\hoctap.exe`
// "Send Nguyen Cao Tri fdsssssssssssssssssssssssssssssssssssssssdsfdsfsdfsdfsd"
// PS C:\Users\trinc\hoctap> cargo run
// Finished dev [unoptimized + debuginfo] target(s) in 0.01s
// Running `target\debug\hoctap.exe`
// "Send Nguyen Cao Tri fdsssssssssssssssssssssssssssssssssssssssdsfdsfsdfsdfsd"
// PS C:\Users\trinc\hoctap> cargo run
// Finished dev [unoptimized + debuginfo] target(s) in 0.01s
// Running `target\debug\hoctap.exe`
// "And Send"

// Sử dụng Join-handle để cho nó đợi  

// use std::sync::mpsc:: channel;
// use std::thread::spawn;
// use std::vec;

// fn main() {
//     let (sender, receiver) = channel();

//     let sender_clone = sender.clone();

//     let mut handle_vec = vec![];

//     handle_vec.push(spawn(move || { sender.send("Send Nguyen Cao Tri fdsssssssssssssssssssssssssssssssssssssssdsfdsfsdfsdfsd").unwrap(); }));

//     handle_vec.push(spawn(move || { sender_clone.send("And Send").unwrap(); }));

//     for _ in handle_vec { println!("{:?}", receiver.recv().unwrap()); }
// }

//-------------------------------------------------------------------------------------------------------

// Ví dụ ta có 1 vec có 1 triệu số 0 , ta muốn thay đổi 0 thành 1 , do khối lượng công việc lớn ta chia làm 10 luồng để cho nó ổn định thì 

use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let(sender,receiver) = channel();

    let vec1tr = vec![0; 1_000_000];

    let mut newvec = vec![];

    let mut handle_vec = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();

        let mut work: Vec<u8> = Vec::with_capacity(vec1tr.len()/10);

        work.extend(&vec1tr[i*100_000..(i+1)*100_000]);

        let handle = spawn(move || {
            for number in work.iter_mut() {
                *number += 1;
            }
            sender_clone.send(work).unwrap();
        });
        handle_vec.push(handle);
    }

    for handle in handle_vec {
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        newvec.push(results);
    }

    let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>();

    println!("{:?}, {:?}, total length: {}",
    &newvec[0..10], &newvec[newvec.len()-10..newvec.len()], newvec.len()
    );

    for number in newvec {
        if number != 1 {
            panic!();
        }
    }
}
