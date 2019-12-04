use std::fs;

const GRID_SIZE: usize = 50000;
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
    pub red:   bool,
    pub green: bool
}

impl Default for GridContent {
    fn default() -> Self {
        Self {
            red:   false,
            green: false
        }
    }
}

fn perform_movement(
    x: &mut usize,
    y: &mut usize,
    mut movement: Movement,
    grid: &mut Vec<Vec<GridContent>>,
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
        movement.amount -= 1;

        grid[*y][*x].red |= colour_red;
        grid[*y][*x].green |= !colour_red;
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
    let mut grid = vec![vec![GridContent::default(); GRID_SIZE]; GRID_SIZE];
    let mut x = PORT_X;
    let mut y = PORT_Y;

    // Perform the red movements
    for movement in red_movements {
        perform_movement(&mut x, &mut y, movement, &mut grid, true);
    }

    x = PORT_X;
    y = PORT_Y;

    // Perform the green movements
    for movement in green_movements {
        perform_movement(&mut x, &mut y, movement, &mut grid, false);
    }

    println!("Intersections: ");
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if grid[y][x].red && grid[y][x].green {
                println!(
                    "{} manhattan units from the port is [{}, {}]",
                    manhattan_distance((x, y), (PORT_X, PORT_Y)),
                    x,
                    y
                );
            }
        }
    }
}
