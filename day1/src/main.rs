use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_for_mass(mass: u64) -> u64 {
    let third_of_mass = mass / 3;

    if third_of_mass > 2 {
        third_of_mass - 2
    }
    else {
        0
    }
}

fn main() {
    let file = File::open("input").expect("Could not open input file");
    let reader = BufReader::new(&file);

    let mut total_mass: u64 = 0;
    let mut total_fuel_required: u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let module_mass = line.parse::<u64>().expect("Module mass was not int");
        let mut fuel_required = fuel_for_mass(module_mass);

        let mut fuel_for_fuel_added = fuel_required;
        while fuel_for_fuel_added != 0 {
            fuel_for_fuel_added = fuel_for_mass(fuel_for_fuel_added);
            fuel_required += fuel_for_fuel_added;
        }

        total_mass += module_mass;
        total_fuel_required += fuel_required;
    }

    println!("Total mass is: {}", total_mass);
    println!("Total fuel required is: {}", total_fuel_required);
}
