pub mod image;

use image::Image;
use std::fs;

pub const IMAGE_WIDTH: usize = 25;
pub const IMAGE_HEIGHT: usize = 6;

fn count_occurences<T>(data: &Vec<Vec<T>>, to_count: T) -> usize
where
    T: PartialEq,
{
    let mut occurences = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if to_count == data[y][x] {
                occurences += 1;
            }
        }
    }

    occurences
}

fn find_layer_with_least_zeroes(image: &Image) -> usize {
    let mut current_least_counted = usize::max_value();
    let mut least_zeroes_layer = None;
    for (i, layer) in image.layers().iter().enumerate() {
        let num_zeroes = count_occurences(&layer, 0);

        println!("Layer {} has {} zeroes", i, num_zeroes);

        if num_zeroes < current_least_counted {
            current_least_counted = num_zeroes;
            least_zeroes_layer = Some(i);
        }
    }

    least_zeroes_layer.expect("Failed to get layer with least zeroes")
}

fn main() {
    let input = fs::read_to_string("input").expect("Unable to read input file");
    let image = Image::from_string(input, IMAGE_WIDTH, IMAGE_HEIGHT);

    let least_zeroes_layer = find_layer_with_least_zeroes(&image);
    println!("Layer with the least zeroes is {}", least_zeroes_layer);

    let num_ones = count_occurences(image.layer(least_zeroes_layer), 1);
    let num_twos = count_occurences(image.layer(least_zeroes_layer), 2);

    println!(
        "Number of ones: {}, of twos: {}, multiplied: {}",
        num_ones,
        num_twos,
        num_ones * num_twos
    );

    println!("The resulting picture:");
    image.render_to_screen();
}
