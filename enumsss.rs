enum CarTypes// create a data types
    Hatchback, 
    Sedan,
    Suv,
    Muv
}

fn printCars(car:CarTypes){
  match car{
    CarTypes::Hatchback => {
        println!("small car in a segment");
    }
    CarTypes::Sedan  =>{
        println!(" luxury car in a segment");
    }
    CarTypes::Suv =>{
        println!(" sports utility based ")
    }
    CarTypes:: Muv =>{
        println!(" general purpose ");
    }
}
}


fn main() {
    println!("Hello, world!");
  printCars(CarTypes::Suv);
  printCars(CarTypes::Muv);

}
