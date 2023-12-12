use std::fs;

fn main() {
    let test_files: Vec<_> = fs::read_dir("./testy")
        .unwrap()
        .map(|file| {
            file
                .unwrap()
                .path()
                .display()
                .to_string()
        })
        .collect();

    println!("{test_files:?}");

    // for file_path in &test_files {
    //     println!("\n\n\t===== {} =====", &file_path[6..]);
    //     let img = Image::from_tga(file_path);
    //     println!("original image: ");
    //     img.print_entropy();

    //     let mut all: Vec<(String, f64)> = Vec::new();
    //     let mut red: Vec<(String, f64)> = Vec::new();
    //     let mut green: Vec<(String, f64)> = Vec::new();
    //     let mut blue: Vec<(String, f64)> = Vec::new();

    //     for predictor in [One, Two, Three, Four, Five, Six, Seven, New] {
    //         println!("Predicator {:?}: ", predictor);
    //         let entropies = img.encode(predictor);
    //         println!(
    //             "\tall = {}\n\tr = {}\n\tg = {}\n\tb = {}\n",
    //             entropies.0, entropies.1, entropies.2, entropies.3
    //         );

    //         all.push((format!("{:?}", predictor), entropies.0));
    //         red.push((format!("{:?}", predictor), entropies.1));
    //         green.push((format!("{:?}", predictor), entropies.2));
    //         blue.push((format!("{:?}", predictor), entropies.3));
    //     }

    //     println!(
    //         "Best all: {:?}",
    //         all.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap()
    //     );
    //     println!(
    //         "Best red: {:?}",
    //         red.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap()
    //     );
    //     println!(
    //         "Best green: {:?}",
    //         green.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap()
    //     );
    //     println!(
    //         "Best blue: {:?}",
    //         blue.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap()
    //     );
    //     println!();
    // }
}
