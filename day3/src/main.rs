use std::collections::HashMap;
use std::fs;

const PORT_X: usize = 25000;
const PORT_Y: usize = 25000;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

use Direction::{Down, Left, Right, Up};

#[derive(Debug, Clone)]
struct Movement {
    pub direction: Direction,
    pub amount:    usize
}

impl Movement {
    pub fn from_str(string: &str) -> Movement {
        let (dir, amount) = string.split_at(1);
        let direction = match &dir {
            &"U" => Up,
            &"L" => Left,
            &"D" => Down,
            &"R" => Right,
            other => panic!("'{}' is not a direction", other)
        };

        let amount = amount
            .parse::<usize>()
            .expect("Failed to parse movement. Second part was not an integer");

        Movement { direction, amount }
    }
}

// Content of the grid at a specific position
#[derive(Debug, Clone, Copy, PartialEq)]
struct GridContent {
    pub red:   usize,
    pub green: usize
}

impl Default for GridContent {
    fn default() -> Self {
        Self {
            red:   usize::max_value(),
            green: usize::max_value()
        }
    }
}

fn perform_movement(
    steps: &mut usize,
    x: &mut usize,
    y: &mut usize,
    mut movement: Movement,
    grid: &mut HashMap<(usize, usize), GridContent>,
    colour_red: bool
) {
    println!("Performing movement: {:?}", movement);

    while movement.amount > 0 {
        match movement.direction {
            Up => *y -= 1,
            Left => *x -= 1,
            Down => *y += 1,
            Right => *x += 1
        }
        *steps += 1;
        movement.amount -= 1;

        match grid.get_mut(&(*x, *y)) {
            Some(entry) => {
                if colour_red {
                    entry.red = usize::min(entry.red, *steps)
                }
                else {
                    entry.green = usize::min(entry.green, *steps)
                }
            }
            None => {
                if colour_red {
                    grid.insert(
                        (*x, *y),
                        GridContent {
                            red:   *steps,
                            green: usize::max_value()
                        }
                    );
                }
                else {
                    grid.insert(
                        (*x, *y),
                        GridContent {
                            green: *steps,
                            red:   usize::max_value()
                        }
                    );
                }
            }
        }
    }
}

fn manhattan_distance((from_x, from_y): (usize, usize), (to_x, to_y): (usize, usize)) -> usize {
    let mut distance = 0;
    if from_x > to_x {
        distance += from_x - to_x;
    }
    else {
        distance += to_x - from_x;
    }
    if from_y > to_y {
        distance += from_y - to_y;
    }
    else {
        distance += to_y - from_y;
    }

    distance
}

fn main() {
    // Load the movements
    let input = fs::read_to_string("input").expect("Could not read input file");
    let (red_movements, green_movements) =
        input.split_at(input.find('\n').expect("Input must have two lines"));

    let red_movements: Vec<Movement> = red_movements
        .split(',')
        .map(|x| Movement::from_str(x.trim()))
        .collect();
    let green_movements: Vec<Movement> = green_movements
        .split(',')
        .map(|x| Movement::from_str(x.trim()))
        .collect();

    // Initialise the grid
    let mut grid = HashMap::new();
    let mut x = PORT_X;
    let mut y = PORT_Y;

    // Perform the red movements
    let mut steps = 0;
    for movement in red_movements {
        perform_movement(&mut steps, &mut x, &mut y, movement, &mut grid, true);
    }

    x = PORT_X;
    y = PORT_Y;

    // Perform the green movements
    steps = 0;
    for movement in green_movements {
        perform_movement(&mut steps, &mut x, &mut y, movement, &mut grid, false);
    }

    println!("Intersections: ");
    for ((x, y), v) in grid {
        if v.red != usize::max_value() && v.green != usize::max_value() {
            println!(
                "{} manhattan units from the port is [{}, {}]",
                manhattan_distance((x, y), (PORT_X, PORT_Y)),
                x,
                y
            );

            println!(
                "{} steps for red, {} for green; {} total",
                v.red,
                v.green,
                v.red + v.green
            );
        }
    }
}
