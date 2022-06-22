// Closures in functions

// fn main() {
//     fn all<F> (&mut self, f: F) -> bool    // f: F maybe change my_closure: Closure
//     where 
//         F: Fnmut(self::Item) -> bool,
// }

// Simple signature with a closure 

// fn do_something <F> (f: F)
// where 
//     F: FnOnce(),
// { 
//     f();
// }

// fn main() {
//     let some_vec = vec![9,8,10];
//     do_something(|| {
//         some_vec
//             .into_iter()
//             .for_each(|x| println!("The number is : {}", x));
//     })
// }

#[derive(Debug)]
struct City {
    name: String,
    year: Vec<u32>,
    population: Vec<u32>
}

impl City {
    fn new(name:&str, year: Vec<u32>, population: Vec<u32>) -> Self {
        Self { 
            name: name.to_string(), 
            year: year, 
            population: population,
        }
    }

    fn city_data<F> (&mut self, mut f:F)
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
        {
            f(&mut self.year, &mut self.population)
        }
}
fn main() {
    let years = vec![1300,1400,1500,1600,1700,1800,1900,2000,2100,2222];

    let populations = vec![130000,140000,150000,160000,170000,180000,190000,200000,210000,222222];

    let mut hochiminh_city = City::new("hochiminh_city", years, populations);


    hochiminh_city.city_data(|city_years, city_populations| {
        let new_vec = city_years
                .into_iter()
                .zip(city_populations.into_iter())
                .take(3)
                .collect::<Vec<(_, _)>>();
            println!("{:?}", new_vec);

    });

    hochiminh_city.city_data(|x, y| {
        x.push(2300);
        y.push(230000);
    });

        
    println!("{:?}", hochiminh_city);

    hochiminh_city.city_data(|x, y| {
        let aaa = x.iter().position(|x| *x == 1400);
        if let Some(a) = aaa {
            println!("Xoa du lieu !!!! {} tai {}", x[a], a);

            x.remove(a);
            y.remove(a);
        }
    });

    println!("Du lieu hien tai {:?}", hochiminh_city);

}
