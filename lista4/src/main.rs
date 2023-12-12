mod lib;

use lib::image::{Image, Predictor::*};

fn main() {
    let test_files = vec![
        "testy/example0.tga",
        "testy/example1.tga",
        "testy/example2.tga",
        "testy/example3.tga",
    ];
    for file_path in &test_files {
        println!("\n\n\t===== {} =====", &file_path[6..]);
        let img = Image::from_tga(file_path);
        println!("Obraz oryginalny: ");
        img.print_entropy();

        let mut all: Vec<(String, f64)> = Vec::new();
        let mut red: Vec<(String, f64)> = Vec::new();
        let mut green: Vec<(String, f64)> = Vec::new();
        let mut blue: Vec<(String, f64)> = Vec::new();

        for predictor in [One, Two, Three, Four, Five, Six, Seven, New] {
            println!("\nPredykator {:?}: ", predictor);
            let entropies = img.encode(predictor);
            println!(
                "\tall = {}\n\tr = {}\n\tg = {}\n\tb = {}",
                entropies.3, entropies.0, entropies.1, entropies.2
            );

            all.push((format!("{:?}", predictor), entropies.3));
            red.push((format!("{:?}", predictor), entropies.0));
            green.push((format!("{:?}", predictor), entropies.1));
            blue.push((format!("{:?}", predictor), entropies.2));
        }

        println!(
            "\nBest all:\n\tPredykator: {}\n\twynik = {}",
            all.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            all.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().1
        );
        println!(
            "Best red:\n\tPredykator: {}\n\twynik = {}",
            red.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            red.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().1
        );
        println!(
            "Best green:\n\tPredykator: {}\n\twynik = {}",
            green.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            green.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().1
        );
        println!(
            "Best blue:\n\tPredykator: {}\n\twynik = {}",
            blue.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            blue.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().1
        );
        println!();
    }
}
