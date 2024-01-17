mod bitvec;
pub mod image;
mod pixel;

use bitvec::*;
use image::Image;
use pixel::Pixel;

pub fn encode_from_tga(image: Image, bitoffset: u8) -> BitVec {
    let mut bitvec = BitVec::new();
    for byte in image.header {
        bitvec.push_byte(byte);
    }
    bitvec.push_byte(bitoffset);
    let mut prev_pixel = Pixel::default();
    let mut quantized_difference_pixel = Pixel::default();
    for pixel in image.img.concat() {
        for (idx, (sub_pixel, prev_sub_pixel)) in pixel.zip(prev_pixel).enumerate() {
            quantized_difference_pixel[idx] = (sub_pixel - prev_sub_pixel) >> (8 - bitoffset);
            bitvec.push_k_lsb(quantized_difference_pixel[idx], bitoffset);
            quantized_difference_pixel[idx] = quantized_difference_pixel[idx] << (8 - bitoffset);
        }
        prev_pixel = prev_pixel + quantized_difference_pixel;
    }
    bitvec
}

pub fn decode_from_bin(path: &str) -> Vec<u8> {
    let file = std::fs::read(path).unwrap();
    let header: Vec<u8> = file[..18].iter().cloned().collect();
    let width = u16::from_le_bytes([file[12], file[13]]) as usize;
    let height = u16::from_le_bytes([file[14], file[15]]) as usize;
    let bitoffset = file[18].clone();
    let mut compressed_img = BitVec::from_bytes(file[19..].iter().cloned().collect());
    let mut prev_pixel = Pixel::default();
    let mut decompressed_img = Vec::new();
    let mut out_file = Vec::new();
    for _ in 0..(width * height) {
        let mut r = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {}
                Bit::One => {
                    r += 1 << (n + 8 - bitoffset);
                }
            }
        }
        let mut g = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {}
                Bit::One => {
                    g += 1 << (n + 8 - bitoffset);
                }
            }
        }
        let mut b = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {}
                Bit::One => {
                    b += 1 << (n + 8 - bitoffset);
                }
            }
        }
        let pixel = Pixel::new(r, g, b);
        prev_pixel = prev_pixel + pixel;
        decompressed_img.push(prev_pixel[2]);
        decompressed_img.push(prev_pixel[1]);
        decompressed_img.push(prev_pixel[0]);
    }

    let mut temp = decompressed_img.chunks(width * 3).collect::<Vec<_>>();
    temp.reverse();
    decompressed_img = temp.concat();

    out_file.extend(header);
    out_file.extend(decompressed_img);
    out_file
}

pub fn get_errors(path0: &str, path1: &str) {
    let img0 = image::Image::from_tga_file(path0);
    let img1 = image::Image::from_tga_file(path1);

    let flat0 = img0.get_flat();
    let flat1 = img1.get_flat();

	// Błąd średniokwadratowy
    let mse: f64 = flat1
        .iter()
        .zip(flat0.iter())
        .map(|(original, out)| original.dist(out).pow(2) as f64)
        .sum::<f64>()
        / flat0.len() as f64;
    println!("MSE: {:?}", &mse);

	// Błąd średniokwadratowy dla kolou czerwonego
    let r: f64 = flat1
        .iter()
        .zip(flat0.iter())
        .map(|(original, out)| original[0].abs_diff(out[0]).pow(2) as f64)
        .sum::<f64>()
        / flat0.len() as f64;
    println!("MSE r: {:?}", &r);

	// Błąd średniokwadratowy dla kolou zielonego
    let g: f64 = flat1
        .iter()
        .zip(flat0.iter())
        .map(|(original, out)| original[1].abs_diff(out[1]).pow(2) as f64)
        .sum::<f64>()
        / flat0.len() as f64;
    println!("MSE g: {:?}", &g);

	// Błąd średniokwadratowy dla kolou niebieskiego
    let b: f64 = flat1
        .iter()
        .zip(flat0.iter())
        .map(|(original, out)| original[2].abs_diff(out[2]).pow(2) as f64)
        .sum::<f64>()
        / flat0.len() as f64;
    println!("MSE b: {:?}", &b);

	// Stosunek sygnału do szumu
    let snr = (flat0
        .iter()
        .map(|v| ((v[0] as f64).powi(2) + (v[1] as f64).powi(2) + (v[2] as f64).powi(2)) as f64)
        .sum::<f64>()
        / flat0.len() as f64)
        / mse;
    println!("SNR: {:?}", &snr);
}
