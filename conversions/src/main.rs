use std::convert::TryFrom;
use std::convert::TryInto;


struct Car {
    brand: Option<String>,
    tires: i32,
    seats: Option<i32>,
    driver: Option<String>
}

struct Bus {
    brand: String,
    tires: i32,
    seats: Option<i32>
}

trait Summary<T> {
    fn summarize(self) -> T;
}

impl<Car> Summary<Car> for Car {
    fn summarize(self) -> Car {
        self
    }
}

impl Bus {
    fn new(brand: String, seats: Option<i32>) -> Bus {
        Bus { brand, tires: 8, seats }
    }
}

impl Car{
    fn new(tires: i32) -> Car {
        Car { brand: None, tires, seats: None, driver: None }
    }
}

impl TryFrom<Bus> for Car {
    type Error = &'static str;

    fn try_from(value: Bus) -> Result<Self, Self::Error> {
        match value.seats {
            Some(s) => {
                let car = Car {
                    seats: Some(s),
                    driver: None,
                    brand: Some(value.brand),
                    tires: value.tires
                };
                Ok(car)
            }
            _ => Err("Bus seats must be specified")
        }
    }
}


fn main() {
    let bus = Bus::new(String::from("Kenwood"), Some(4));
    let bus_to_car = Car::try_from(bus);
    if let Ok(car) = bus_to_car {
        println!("This is a car with {:?} seats", car.seats)
    }

    let bus2 = Bus::new(String::from("Foton"), None);
    let bus_to_car2 = Car::try_from(bus2);
    if let Err(err) = bus_to_car2 {
        println!("{}", err)
    }



    let car2_to_bus2 = bus_to_car2.try_into();
}

    
