// trait Vehicle {
//     fn drive(&self);
// }

// struct Truck {
//     next_truck: Option<Box<Truck>>,
// }

// impl Vehicle for Truck {
//     fn drive(&self) {
//         println!("Truck driving");
//     }
// }

use std::{rc::Rc, thread, sync::Arc};

#[derive(Debug)]
struct Truck {
    capacity: i32
}

fn main() {
    let (truck_a,
        truck_b, 
        truck_c) = (
            Arc::new(Truck { capacity: 1 }), 
            Arc::new(Truck { capacity: 2 }), 
            Arc::new(Truck { capacity: 3 })
        );

    let thread = thread::spawn(move || {
        let facility_one = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility_two = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];    
        (facility_one, facility_two)
    });

    let (facility_one, facility_two) = thread.join().unwrap();

    let truck_b = Arc::clone(&facility_one[1]);
    println!("Truck b strong count: {:?}", Arc::strong_count(&truck_b));

    std::mem::drop(facility_one);

    println!("Facility two after dropping one: {:?}", facility_two);
}
