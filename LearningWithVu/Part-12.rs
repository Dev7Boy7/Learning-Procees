
// fn caps(input: &str) -> String {
//     input.to_uppercase()
// }
// fn main() {}

// #[cfg(test)]
// mod test {
//     use crate::*;

//     #[test]
//     fn check() {
//         let result = caps("nguyen cao tri");
//         let expected = String::from("NGUYEN CAO TRI");
//         assert_eq!(result, expected, "String should be all uppercase");
//     }

//     // fn check_uppercase() {
//     //     let result = caps("le xuan tien");
//     //     let expected = String::from("LE XUAN TIEN");
//     //     assert_eq!(result, expected, "string should be all uppercase");
//     // }
// }

// INPUT / OUTPUT

//

// fn main() {
//     let abc = vec![1,2,3,4,5];

//     let mut aaa = vec![];

//     for number in abc {
//         aaa.push(number+1);
//         println!("number la : {}", number);
//     }

//     println!("Vector moi la : {:?}", aaa)
// }

//------------------------------------------------------------------------------------

// fn main() {
//     let acb = vec![1,2,3,4,5];

//     let aaa = acb
//                     .iter() 
//                     .map( |x| {
//                         println!("number la: {}", x);
//                         x+1
//                     })
//                     .collect::<Vec<i32>>();

//     println!("Vector moi la : {:?}", aaa)
// }

//------------------------------------------------------------------------------------

// fn main() {
//     let acb = vec![1,2,3,4,5];

//     let aaa = acb
//                 .iter()
//                 .map( |x| x * 2 )
//                 .filter(|x| x > &6)
//                 .collect::<Vec<i32>>();

//     println!("{:?}", aaa)
// }

//--------------------------------------------------------------------------------------

// Closure 

// fn main() {
//     let cong2so = |x : i32, y: i32| x+y ;

//     let abc = cong2so(2,5);

//     let aaa = |x,y,z| x + y+ z;

//     println!("{}", abc);

//     println!("{}", aaa(10,10,20));

// }

//------------------------------------------------------------------------------------------

fn math(a:i32, b:i32, ob: Box<dyn Fn(i32,i32) -> i32>) -> i32{
    ob(a,b)
}

fn main(){
    let add = |a,b| a+b;
    let multi = |a,b| a*b;

    let add = Box::new(add);
    let multi = Box::new(multi);

    println!("{}", math(2,3,add));
    println!("{}", math(2,6,multi));

}
