mod lib;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, CellAlignment, ContentArrangement, Table,
};
use lib::image::{Image, Predictor::*};
use std::fs;

fn main() {
    let test_files: Vec<_> = fs::read_dir("./testy")
        .unwrap()
        .map(|file| file.unwrap().path().display().to_string())
        .collect();

    for file_path in &test_files {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);

        println!("\n\n\t===== {} =====", &file_path[8..]);
        table.set_header(vec!["", "All", "red", "green", "blue"]);
        let img = Image::from_tga(file_path);
        let (r, g, b, a) = img.entropy();
        table.add_row(vec![
            "Original image".to_string(),
            format!("{:.4}", a),
            format!("{:.4}", r),
            format!("{:.4}", g),
            format!("{:.4}", b),
        ]);

        let mut all: Vec<(String, f64)> = Vec::new();
        let mut red: Vec<(String, f64)> = Vec::new();
        let mut green: Vec<(String, f64)> = Vec::new();
        let mut blue: Vec<(String, f64)> = Vec::new();

        for predictor in [One, Two, Three, Four, Five, Six, Seven, New] {
            let (r, g, b, a) = img.encode(predictor);
            table.add_row(vec![
                format!("{:?}", predictor),
                format!("{:.4}", a),
                format!("{:.4}", r),
                format!("{:.4}", g),
                format!("{:.4}", b),
            ]);

            all.push((format!("{:?}", predictor), a));
            red.push((format!("{:?}", predictor), r));
            green.push((format!("{:?}", predictor), g));
            blue.push((format!("{:?}", predictor), b));
        }
        table.add_row(vec![
            "Best".to_string(),
            format!(
                "{:.4}",
                all.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            ),
            format!(
                "{:.4}",
                red.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            ),
            format!(
                "{:.4}",
                green.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0,
            ),
            format!(
                "{:.4}",
                blue.iter().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap().0
            ),
        ]);

        print!("{table}");
    }
}
