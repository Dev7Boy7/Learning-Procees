
// Arc 

// Nhớ lại bài học trước về Rc , khi muốn làm điều tương tự với Thread ta sử dụng Arc - Atomic reference counter 

// when 2 threads run at the same time , u could get wrong result . Example :

fn main() {
    let mut x = 100;

    for i in 0..20 { x += 1 };
    for i in 0..5 { x = x*2 };

    println!("{}", x)
}

// an Arc uses the processor to make sure this doesnt happen

//------------------------------------------------------------------------------

fn main() {
    let handle = std::thread::spawn( || {
        println!("Working")
    });

    handle.join().unwrap();
    println!("End");
}

//--------------------------------------------------------------------------------

fn main() {

    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1 is working!")
        }
    });

    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working!")
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exiting the program");
}

// sometimes we saw Thread1 1st sometimes Thread2 .  Concurrency - running together 

// xem xét một ví dụ sau 

use std::sync::{Arc, Mutex};
use std::thread::spawn; // Now we just write spawn

fn make_arc(number: i32) -> Arc<Mutex<i32>> { // Just a function to make a Mutex in an Arc
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> { // Just a function so we can write new_clone
    Arc::clone(&input)
}

// Now main() is easier to read
fn main() {
    let mut handle_vec = vec![]; // each handle will go in here
    let my_number = make_arc(0);

    for _ in 0..2 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);    // the handle is done, so put it in the vector
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // Make each one wait

    println!("{:?}", my_number);
}
