// giving a name to another type

// when have a very long type and don't want to write it every time

fn returns<'a>(input: &'a Vec<char>) -> std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>> {
    input.iter().skip(4).take(5)
}
fn main() {}
  
    
type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;
fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}
fn main() {}
  
  
use std::iter::{Take, Skip};
use std::slice::Iter;
fn returns<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
    input.iter().skip(4).take(5)
}
fn main() {}
  
//Importing and renaming inside a function
  
example: 
  enum Giadinh {
    Nguyencaotri,
    Lexuantien,
    Nguyenthanhtai,
}

fn main(){
    party_meeting(&Giadinh::Lexuantien);
}

// fn party_meeting(aaa: &Giadinh) {
//     match aaa {
//         Giadinh::Nguyencaotri => println!("welcome, tricao"),
//         Giadinh::Lexuantien => println!("welcome, tiendau"),
//         Giadinh::Nguyenthanhtai => println!("welcome,bi"),
//     }
// }

fn party_meeting(aaa: &Giadinh) {
    use Giadinh::*;
    match aaa {
        Nguyencaotri => println!("welcome, tricao"),
        Lexuantien => println!("welcome, tiendau"),
        Nguyenthanhtai => println!("welcome,bi"),
    }
}
  
// You can also use as to change the name
  
enum FileState {
    CannotAccessFile,
    ...,
}

fn give_filestate(input: &FileState) {
    use FileState::{
        CannotAccessFile as NoAccess,
        ...
    };
      
    match input {
       NoAccess => println!("Can't access file."),
       ...
    }
}

fn main() {}
