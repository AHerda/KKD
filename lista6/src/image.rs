use crate::pixel::{pixel_from_bgr, Pixel};
use entropy;

use image::{ImageBuffer, RgbImage};
use log::{info, debug};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub img: Vec<Vec<Pixel>>,
    pub header: Vec<u8>,
    pub footer: Vec<u8>,
}

impl Image {
    pub fn from_tga_file(path: &str) -> Image {
        let file = std::fs::read(path).unwrap();
        Self::from_tga(&file)
    }

    pub fn from_tga(content: &[u8]) -> Image {
        let width = usize::from(u16::from_le_bytes([content[12], content[13]]));
        let height = usize::from(u16::from_le_bytes([content[14], content[15]]));
        let img_bytes = &content[18..(3 * width * height + 18)];
        let depth = content[16];

        debug!("width: {}", &width);
        debug!("height: {}", &height);
        debug!("depth: {}", &depth);
        debug!("image size: {}B", img_bytes.len());

        let header: Vec<u8> = content[..18].to_vec();
        let footer: Vec<u8> = content[(3 * width * height + 18)..].to_vec();

        let mut img = img_bytes
            .chunks(3)
            .map(|pixel| pixel_from_bgr(pixel).unwrap())
            .collect::<Vec<Pixel>>()
            .chunks(width)
            .map(|v| Vec::from(v))
            .collect::<Vec<Vec<Pixel>>>();

        img.reverse();

        Image {
            width,
            height,
            img,
            header,
            footer,
        }
    }

    pub fn save_as_png(&self, path: &str) {
        let mut img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let rgb = self.img[y as usize][x as usize].to_bytes_rgb();
            *pixel = image::Rgb([rgb[0], rgb[1], rgb[2]]);
        }

        info!("Saving image to {}... ", path);
        img.save(path).unwrap();
    }

    /// Calculates entropy of red, blue and green  part of pixels and also of whole pixels.
    /// The entropy is returned in the form of tuple
    /// ### Returns
    /// (red, green, blue, all)
    pub fn entropy(&self) -> (f64, f64, f64, f64) {
        Self::entropy2(&self.img)
    }

    fn entropy2(pixels: &Vec<Vec<Pixel>>) -> (f64, f64, f64, f64) {
        let r = entropy::entropy(
            &pixels
                .concat()
                .par_iter()
                .map(|x: &Pixel| x[0])
                .collect::<Vec<u8>>(),
        );
        let g = entropy::entropy(
            &pixels
                .concat()
                .par_iter()
                .map(|x: &Pixel| x[1])
                .collect::<Vec<u8>>(),
        );
        let b = entropy::entropy(
            &pixels
                .concat()
                .par_iter()
                .map(|x: &Pixel| x[2])
                .collect::<Vec<u8>>(),
        );
        let all = entropy::entropy(&pixels.concat());
        (r, g, b, all)
    }

    pub fn print_entropy(&self) {
        let (r, g, b, all) = self.entropy();
        println!("\tall = {}\n\tr = {}\n\tg = {}\n\tb = {}", all, r, g, b);
    }

    pub fn get_flat(&self) -> Vec<Pixel> {
        self.img.concat()
    }
}

fn avg_vec(vecs: &[Pixel]) -> Pixel {
    let (sum, count) = vecs
        .par_iter()
        .map(|&pixel| ((pixel[0] as u64, pixel[1] as u64, pixel[2] as u64), 1u64))
        .reduce(
            || ((0, 0, 0), 0),
            |a, b| {
                (
                    (a.0 .0 + b.0 .0, a.0 .1 + b.0 .1, a.0 .2 + b.0 .2),
                    a.1 + b.1,
                )
            },
        );

    let size = count as f64;
    Pixel::new(
        (sum.0 as f64 / size) as u8,
        (sum.1 as f64 / size) as u8,
        (sum.2 as f64 / size) as u8,
    )
}
