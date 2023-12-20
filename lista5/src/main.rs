mod lib;

use std::{env, fs::File, io::Write};

use lib::image::Image;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Wrong argument count")
    }

    let input_path = args.get(1).unwrap();
    let output_path = args.get(2).unwrap();
    let color_count: usize = args.get(3).unwrap().parse().unwrap();

    let img = Image::from_tga_file(input_path);
    // print!("{}", input_path);
    img.save_as_png(
        format!(
            "./pics/originals/{}.png",
            &input_path[6..input_path.len() - 4]
        )
        .as_str(),
    );

    let codebook = img.quantization(color_count);
        let quantized_img = img.codebook_to_tga(&codebook);

    let mut output = File::create(output_path).unwrap();
        output.write_all(&quantized_img).unwrap();

    let start = output_path.find("/").unwrap_or(usize::MAX).wrapping_add(1);
    let end = output_path.len() - 4;
    let raw_name = &output_path[start..end];

    Image::from_tga(&quantized_img).save_as_png(format!("pics/{}.png", raw_name).as_str());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn imige_ingest_test() {
        for path in [
            "testy/example0.tga",
            "testy/example1.tga",
            "testy/example2.tga",
            "testy/example3.tga",
        ] {
            Image::from_tga_file(path);
        }
    }
    #[test]
    fn color_test() {
        let img = Image::from_tga_file("testy/example1.tga");
        assert_eq!(img.img[0][0], lib::pixel::Pixel::new(0xFF, 0, 0xFF));
        assert_eq!(img.img[img.height - 1][0], lib::pixel::Pixel::new(0, 0, 0));
        assert_eq!(
            img.img[0][img.width - 1],
            lib::pixel::Pixel::new(0xFE, 0xFF, 0xFF)
        );
        assert_eq!(
            img.img[img.height - 1][img.width - 1],
            lib::pixel::Pixel::new(0xFF, 0xFF, 0)
        );
    }
    #[test]
    fn quantization_test() {
        for path in [
            "testy/example0.tga",
            "testy/example1.tga",
            "testy/example2.tga",
            "testy/example3.tga",
        ] {
            let img = Image::from_tga_file(path);
            for i in 1..5 {
                let result = img.quantization(i);
                assert_eq!(result.len(), 2_usize.pow(i as u32));
            }
        }
    }
}
