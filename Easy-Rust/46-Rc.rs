
// RC - reference counter 

// Mỗi variable chỉ có thể 1 chủ sở hữu 

//Rc writes down who has ownership, and how many 

#[derive(Debug)]
struct quan_huyen {
    ten_quan_huyen : String,
    dan_so : u32,
    lich_su_quan_huyen : String,
}
#[derive(Debug)]
struct dulieu_quanhuyen {
    name: Vec<String>,
    lich_su : Vec<String>,
}

fn main() {
    let quan_binh_tan = quan_huyen {
        ten_quan_huyen: "Binh Tan".to_string(),
        dan_so : 1_000_000,
        lich_su_quan_huyen: "Tach ra tu binh chanh".to_string(),
    };

    let binh_tan = dulieu_quanhuyen {
        name : vec![quan_binh_tan.ten_quan_huyen], // dữ liệu đã được chuyển đến đây
        lich_su : vec![quan_binh_tan.lich_su_quan_huyen], // dữ liệu đã được chuyển đến đây
    };

    // println!("{:?}", quan_binh_tan);

    println!("{:?}", binh_tan);
    
//--------------------------------------------------------------------------------------------- 

use std::rc::Rc;

#[derive(Debug)]
struct quan_huyen {
    ten_quan_huyen : String,
    dan_so : u32,
    lich_su_quan_huyen : Rc<String>,
}
#[derive(Debug)]
struct dulieu_quanhuyen {
    name: Vec<String>,
    lich_su : Vec<Rc<String>>,
}

fn main() {
    let quan_binh_tan = quan_huyen {
        ten_quan_huyen: "Binh Tan".to_string(),
        dan_so : 1_000_000,
        lich_su_quan_huyen: Rc::new("Tach ra tu binh chanh".to_string()),
    };

    let binh_tan = dulieu_quanhuyen {
        name : vec![quan_binh_tan.ten_quan_huyen], // dữ liệu đã được chuyển đến đây
        lich_su : vec![quan_binh_tan.lich_su_quan_huyen.clone()], // dữ liệu đã được chuyển đến đây
    };

    println!("{:?}", quan_binh_tan.lich_su_quan_huyen);

    println!("{:?}", Rc::strong_count(&quan_binh_tan.lich_su_quan_huyen));

    let new_owner = quan_binh_tan.lich_su_quan_huyen.clone();

    println!("{:?}", Rc::strong_count(&quan_binh_tan.lich_su_quan_huyen));
}

// Kiểm tra số lượng sở hữu Rc::strong_count(&item)
