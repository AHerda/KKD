use super::pixel::{pixel_from_bgr, Pixel};
use entropy;

use image::{ImageBuffer, RgbImage};
use log::info;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

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

        info!("width: {}", &width);
        info!("height: {}", &height);
        info!("depth: {}", &depth);
        info!("image size: {}B", img_bytes.len());

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

        println!("Saving image to {}... ", path);
        img.save(path).unwrap();
    }

    pub fn quantization(&self, color_count: usize) -> Vec<Pixel> {
        let cluster_count = 2_usize.pow(color_count as u32);

        let training_vectors: Vec<Pixel> = self.img.concat().clone();
        let mut codebook: Vec<Pixel> = Vec::new();
        let c_0 = avg_vec(&training_vectors);

        codebook.push(c_0);
        while codebook.len() < cluster_count {
            codebook = lgb(&training_vectors, &codebook);
        }
        codebook
    }

    pub fn codebook_to_tga(&self, codebook: &[Pixel]) -> Vec<u8> {
        let mut temp = self.img.clone();
        temp.reverse();
        let vectors: Vec<Pixel> = temp.concat();

        let mut output = Vec::new();
        let mut out_vector = Vec::new();

        output.append(&mut self.header.clone());

        for vector in &vectors {
            let coded = codebook.par_iter().min_by_key(|c| c.dist(&vector)).unwrap();

            out_vector.push(coded);
            output.append(&mut coded.to_bytes_brg());
        }

        let mse: f64 = out_vector
            .par_iter()
            .zip(vectors.par_iter())
            .map(|(original, out)| original.dist(out).pow(2) as f64)
            .sum::<f64>()
            / vectors.len() as f64;

        let snr = (vectors
            .par_iter()
            .map(|v| (v[0].pow(2) + v[1].pow(2) + v[2].pow(2)) as f64)
            .sum::<f64>()
            / vectors.len() as f64)
            / mse;

        println!("MSE: {:?}", &mse);
        println!("SNR: {:?}", &snr);

        output.append(&mut self.footer.clone());
        output
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
}

fn avg_vec(vecs: &[Pixel]) -> Pixel {
    let (sum, count) = vecs
        .par_iter()
        .map(|&pixel| ((pixel[0] as u64, pixel[1] as u64, pixel[2] as u64), 1u64))
        .reduce(|| ((0, 0, 0), 0), |a, b| ((a.0 .0 + b.0 .0, a.0 .1 + b.0 .1, a.0 .2 + b.0 .2), a.1 + b.1));

    let size = count as f64;
    Pixel::new((sum.0 as f64 / size) as u8, (sum.1 as f64 / size) as u8, (sum.2 as f64 / size) as u8)
}

// fn avg_vec(vecs: &[Pixel]) -> Pixel {
//     let size = vecs.len() as f64;
//     let mut avg_vec = (0_f64, 0_f64, 0_f64);
//     for vector in vecs {
//         avg_vec.0 += vector[0] as f64 / size;
//         avg_vec.1 += vector[1] as f64 / size;
//         avg_vec.2 += vector[2] as f64 / size;
//     }
//     Pixel::new(avg_vec.0 as u8, avg_vec.1 as u8, avg_vec.2 as u8)
// }


fn lgb(training_vectors: &[Pixel], codebook: &Vec<Pixel>) -> Vec<Pixel> {
    let mut prev_distortion = 0;
    let mut new_codebook = Vec::new();

    for c in codebook {
        let perturbation = c.perturbation(1);
        new_codebook.push(perturbation.0);
        new_codebook.push(perturbation.1);
    }

    loop {
        let mut clusters: Vec<Vec<Pixel>> = vec![Vec::new(); new_codebook.len()];
        for vector in training_vectors {
            let assignment = new_codebook
                .iter()
                .map(|centroid| vector.dist(centroid))
                .enumerate()
                .min_by_key(|(_idx, dist)| *dist)
                .unwrap()
                .0;
            clusters[assignment].push(*vector);
        }
        let mut current_distortion = 0;
        for (idx, cluster) in clusters.iter().enumerate() {
            current_distortion += cluster
                .iter()
                .map(|vector| new_codebook[idx].dist(vector))
                .sum::<usize>();
        }
        if ((current_distortion as f64 - prev_distortion as f64) / current_distortion as f64).abs()
            < 0.0001
        {
            break;
        }
        prev_distortion = current_distortion;
        new_codebook = clusters.par_iter().map(|cluster| avg_vec(cluster)).collect();
    }
    new_codebook
}
