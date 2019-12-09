use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("Could not open input file");
    let reader = BufReader::new(&file);

    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(')').map(|x| x.trim()).collect();
        let center = split[0];
        let orbiter = split[1];

        if orbits.contains_key(center) {
            orbits.get_mut(center).unwrap().push(String::from(orbiter));
        } else {
            orbits.insert(String::from(center), vec![String::from(orbiter)]);
        }
    }

    while {} {}

    println!("Direct orbits: {:#?}", direct_orbits);
}
