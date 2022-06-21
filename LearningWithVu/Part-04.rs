// enum - Option

// #[derive(Debug)]
// enum IpAddressKind {
//     V4,
//     V6
// }

// #[derive(Debug)]
// struct _IpAddress {
//     kind : IpAddressKind,
//     address : String,
// }
// fn main(){
//     let four = IpAddressKind::V4;
//     let six = IpAddressKind::V6;

//     let localhost = _IpAddress {
//         kind: IpAddressKind::V4,
//         address: "127.0.0.1".to_string(),
//     };

//     println!("Local host = {:?}", localhost);
// }

// fn route ( ipkind: IpAddressKind) {}

// #[derive(Debug)]
// enum IpAddressKind {
//     V4(String),
//     V6(String),
// }

// // #[derive(Debug)]
// // struct _IpAddress {
// //     kind : IpAddressKind,
// //     address : String,
// // }
// fn main(){
//     let localhost = IpAddressKind::V4("127.0.0.1".to_string());

//     println!("Local host = {:?}", localhost);
// }

// fn route ( ipkind: IpAddressKind) {}

// #[derive(Debug)]
// enum IpAddressKind {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// // #[derive(Debug)]
// // struct _IpAddress {
// //     kind : IpAddressKind,
// //     address : String,
// // }
// fn main(){
//     let localhost = IpAddressKind::V4(127,0,0,1);

//     println!("Local host = {:?}", localhost);
// }
