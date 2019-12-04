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
        let fuel_required = fuel_for_mass(module_mass);

        total_mass += module_mass;
        total_fuel_required += fuel_required;
    }

    println!("Total mass is: {}", total_mass);
    println!("Total fuel required is: {}", total_fuel_required);

    println!("Calculating fuel for fuel.");

    let mut fuel_added = total_fuel_required;
    while fuel_added != 0 {
        fuel_added = fuel_for_mass(fuel_added);
        total_fuel_required += fuel_added;

        println!("Adding {} fuel for the added fuel", fuel_added);
    }

    println!(
        "Total fuel after accounting for additional fuel: {}",
        total_fuel_required
    );
}
