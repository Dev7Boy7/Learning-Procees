// Multiple threads 

// Os thread - the operating system creates the thread on a different core

// Vd :

fn main() {
    std::thread::spawn(|| {
        println!("Nguyen cao tri");
    });
}

// Sẽ có lúc in lúc không bởi vì thỉnh thoảng main() sẽ xong trước khi thread . 

fn main() {
    for _ in 1..20 {
    std::thread::spawn(|| {
        println!("Nguyen cao tri");
    });
    }
}

// joinHandle

fn main() {
    for _ in 1..20 {
        let _handle = std::thread::spawn(|| {
        println!("Nguyen cao tri");
        });

        _handle.join(); // it will wait for each of the threads to finish
    }
}

// three types of closures : FnOnce, Fnmut, Fn.

// Fn ------------------------------------------------------------------------------------

fn main() {
    let my_string = String::from("Toi la Nguyen Cao Tri");

    let my_closure =  || println!("{}", my_string);

    my_closure();

    my_closure();
}

// Fnmut ----------------------------------------------------------------------------------

fn main() {
    let mut my_string = String::from("Toi la Nguyen Cao Tri");

    let mut my_closure =  || {
        my_string.push_str(" now");
        println!("{}", my_string);
    };

    my_closure();

    my_closure();

    my_closure();
}

// FnOnce ----------------------------------------------------------------------------------

fn main() {
    let my_vec: Vec<i32> = vec![8, 9, 10];
    
    let my_closure = || {
        my_vec
            .into_iter() // into_iter takes ownership
            .map(|x| x as u8) // turn it into u8
            .map(|x| x * 2) // multiply by 2
            .collect::<Vec<u8>>() // collect into a Vec
    };
    
    let new_vec = my_closure();
    println!("{:?}", new_vec);
}

//---------------------------------------------------------------------------------------------

fn main() {
    let my_string = String::from("Can I go inside the thread?");

    let handle = std::thread::spawn(move || {           // move 
        println!("{}", my_string);
    });

    handle.join().unwrap();
}




