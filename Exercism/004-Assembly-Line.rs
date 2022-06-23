#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let a :f64 = 221.0;
    if speed > 0 && speed <= 4  { a * speed as f64  }
    else if speed >4 && speed <=8  { a * 0.9 * speed as f64 }
    else { a * 0.77 * speed as f64 }
    }
                                 
pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60 as u32
}
