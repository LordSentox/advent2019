#[derive(Debug, Clone)]
pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<Vec<u8>>>,
}

impl Image {
    pub fn from_string(data: String, width: usize, height: usize) -> Self {
        let values: Vec<u8> = data
            .trim()
            .chars()
            .map(|x| x.to_digit(10).expect("Input data corrupt") as u8)
            .collect();

        let mut layers = Vec::new();

        // Fill the layers of the image with information
        for l in 0..(data.len() / (width * height)) {
            let mut layer = vec![Vec::with_capacity(width); height];
            for y in 0..height {
                for x in 0..width {
                    layer[y].push(values[(l * width * height) + (y * width) + x]);
                }
            }
            layers.push(layer);
        }

        println!("{} layers have been read", layers.len());

        Self {
            width,
            height,
            layers,
        }
    }

    pub fn layers(&self) -> &Vec<Vec<Vec<u8>>> {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut Vec<Vec<Vec<u8>>> {
        &mut self.layers
    }

    pub fn layer(&self, layer: usize) -> &Vec<Vec<u8>> {
        &self.layers[layer]
    }

    pub fn render_to_screen(&self) {
        const TRANSPARENCY: u8 = 2;
        let mut res = vec![vec![TRANSPARENCY; self.width]; self.height];

        for layer in &self.layers {
            for y in 0..self.height {
                for x in 0..self.width {
                    res[y][x] = match res[y][x] {
                        TRANSPARENCY => layer[y][x],
                        colour => colour,
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                match res[y][x] {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => print!(" "),
                }
            }

            println!();
        }
    }
}
