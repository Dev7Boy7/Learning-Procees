
// Reading Rust Documentation

// assert_eq!
// Doing testing - program will panic if they not equal 

fn testing(input: u8) {
    assert_eq!(input%2, 0);
    println!("The number is not odd. It is {}", input)
}
fn main() {
    testing(60);

    testing(51);
}

// Searching 

// Information on traits
