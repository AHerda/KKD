use log::info;
use std::fmt::Error;
use entropy;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub type Pixel = [u8; 3];

#[derive(Debug, Clone, Copy)]
pub enum Predictor {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    New
}

fn pixel_from(colors: &[u8]) -> Result<Pixel, Error> {
    if colors.len() != 3 {
        Err(Error)
    } else {
        Ok([colors[0], colors[1], colors[2]])
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub img: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn from_tga(path: &str) -> Image {
        let file = std::fs::read(path).unwrap();
        let width = usize::from(u16::from_le_bytes([file[12], file[13]]));
        let height = usize::from(u16::from_le_bytes([file[14], file[15]]));
        let img_bytes = &file[18..(3 * width * height + 18)];
        let depth = file[16];

        info!("width: {}", &width);
        info!("height: {}", &height);
        info!("depth: {}", &depth);
        info!("image size: {}B", img_bytes.len());

        let img = img_bytes
            .chunks(3)
            .map(|pixel| pixel_from(pixel).unwrap())
            .collect::<Vec<Pixel>>()
            .chunks(width)
            .map(|v| Vec::from(v))
            .collect::<Vec<Vec<Pixel>>>();

        Image { width, height, img }
    }

    pub fn encode(&self, predictor: Predictor) -> (f64, f64, f64, f64) {
        let prediction = match predictor {
            Predictor::One => self.predicton_1(),
            Predictor::Two => self.predicton_2(),
            Predictor::Three => self.predicton_3(),
            Predictor::Four => self.predicton_4(),
            Predictor::Five => self.predicton_5(),
            Predictor::Six => self.predicton_6(),
            Predictor::Seven => self.predicton_7(),
            Predictor::New => self.predicton_new(),
        };
        let diff = self.diff(prediction);
        Self::entropy2(&diff)
    }

    pub fn diff(&self, prediction: Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
        let mut diff: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let red_diff = self.img[y][x][0].abs_diff(prediction[y][x][0]);
                let green_diff = self.img[y][x][1].abs_diff(prediction[y][x][1]);
                let blue_diff = self.img[y][x][2].abs_diff(prediction[y][x][2]);
                diff[y][x] =  [red_diff, green_diff, blue_diff];
            }
        }
        diff
    }

    fn predicton_1(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 0..self.height {
            for x in 1..self.width {
                prediction[y][x] = self.img[y][x - 1];
            }
        }
        prediction
    }

    fn predicton_2(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 0..self.width {
                prediction[y][x] = self.img[y - 1][x];
            }
        }
        prediction
    }

    fn predicton_3(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 1..self.width {
                prediction[y][x] = self.img[y - 1][x - 1];
            }
        }
        prediction
    }

    fn predicton_4(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.img[0][x -1];
        }
        for y in 1..self.height {
            prediction[y][0] = self.img[y - 1][0];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.img[y - 1][x];
                let west = self.img[y][x -1];
                let north_west = self.img[y - 1][x];
                prediction[y][x] = [north[0] - north_west[0] + west[0], north[1] - north_west[1] + west[1], north[2] - north_west[2] + west[2]];
            }
        }
        prediction
    }

    fn predicton_5(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            let west = self.img[0][x - 1];
            prediction[0][x] = [west[0]/2, west[1]/2, west[2]/2];
        }
        for y in 1..self.height {
            prediction[y][0] = self.img[y - 1][0];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.img[y - 1][x];
                let west = self.img[y][x -1];
                let north_west = self.img[y - 1][x];
                prediction[y][x] = [north[0] - north_west[0] / 2 + west[0] / 2, north[1] - north_west[1] / 2 + west[1] / 2, north[2] - north_west[2] / 2 + west[2] / 2];
            }
        }
        prediction
    }

    fn predicton_6(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.img[0][x-1];
        }
        for y in 1..self.height {
            let north = self.img[y-1][0];
            prediction[y][0] = [north[0]/2, north[1]/2, north[2]/2];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.img[y - 1][x];
                let west = self.img[y][x -1];
                let north_west = self.img[y - 1][x];
                prediction[y][x] = [west[0] + (north[0] - north_west[0])/2, west[1] + (north[1] - north_west[1])/2, west[2] + (north[2] - north_west[2])/2];
            }
        }
        prediction
    }

    fn predicton_7(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.img[0][x - 1];
        }
        for y in 1..self.height {
            let north = self.img[y][0];
            prediction[y][0] = [north[0]/2, north[1]/2, north[2]/2];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.img[y - 1][x];
                let west = self.img[y][x -1];
                let north_west = self.img[y - 1][x];
                prediction[y][x] = [west[0] + (north[0] - north_west[0])/2, west[1] + (north[1] - north_west[1])/2, west[2] + (north[2] - north_west[2])/2];
            }
        }
        prediction
    }

    fn predicton_new(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 1..self.width {
                let north = if y == 0 {
                    [0, 0, 0]
                } else {
                    self.img[y - 1][x]
                };
                let west = if x == 0 {
                    [0, 0, 0]
                } else {
                    self.img[y][x - 1]
                };
                let north_west = if x == 0 || y ==0 {
                    [0, 0, 0]
                } else {
                    self.img[y - 1][x - 1]
                };
                let mut pixel = [0, 0, 0];
                for c in 0..3 {
                    if north_west[c] >= west[c].max(north[c]) {
                        pixel[c] = west[c].max(north[c]);
                    } else if north_west[c] <= west[c].min(north[c]) {
                        pixel[c] = west[c].min(north[c]);
                    } else {
                        pixel[c] = (north[c] as usize + west[c] as usize - north_west[c] as usize) as u8;
                    }
                }
                prediction[y][x] = pixel;
            }
        }
        prediction
    }

    /// Calculates entropy of red, blue and green  part of pixels and also of whole pixels.
    /// The entropy is returned in the form of tuple
    /// ### Returns
    /// (red, green, blue, all)
    pub fn entropy(&self) -> (f64, f64, f64, f64) {
        let r = entropy::entropy(&self.img.concat().par_iter().map(|x: &Pixel| x[0]).collect::<Vec<u8>>());
        let g = entropy::entropy(&self.img.concat().par_iter().map(|x: &Pixel| x[1]).collect::<Vec<u8>>());
        let b = entropy::entropy(&self.img.concat().par_iter().map(|x: &Pixel| x[2]).collect::<Vec<u8>>());
        let all = entropy::entropy(&self.img.concat());
        (r, g, b, all)
    }

    fn entropy2(pixels: &Vec<Vec<Pixel>>) -> (f64, f64, f64, f64) {
        let r = entropy::entropy(&pixels.concat().par_iter().map(|x: &Pixel| { x[0] }).collect::<Vec<u8>>());
        let g = entropy::entropy(&pixels.concat().par_iter().map(|x: &Pixel| x[1]).collect::<Vec<u8>>());
        let b = entropy::entropy(&pixels.concat().par_iter().map(|x: &Pixel| x[2]).collect::<Vec<u8>>());
        let all = entropy::entropy(&pixels.concat());
        (r, g, b, all)
    }

    pub fn print_entropy(&self) {
        let (r, g, b, all) = self.entropy();
        println!("\tall = {}\n\tr = {}\n\tg = {}\n\tb = {}", all, r, g, b);
    }
}
