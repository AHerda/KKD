mod libs;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, CellAlignment, ContentArrangement, Table,
};
use libs::{exercises, functions, helpers};
use rayon::prelude::*;

fn main() {
    let dane: helpers::Dane = helpers::read_args();
    match dane.path {
        helpers::PathType::Dir(dir_path) => {
            let entropies: Vec<(String, (f64, f64))> = helpers::format_srcs(&dir_path)
                .par_iter()
                .map(|(file_name, file_content)| {
                    ((*file_name).clone(), exercises::zadanie(file_content))
                })
                .collect();

            if dane.table {
                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic);

                table.set_header(vec!["File Name", "Entropy", "Conditional Entropy"]);

                entropies
                    .iter()
                    .for_each(|(file_name, (entropy, cond_entropy))| {
                        table.add_row(vec![
                            format!("{}", file_name),
                            format!("{}", entropy),
                            format!("{}", cond_entropy),
                        ]);
                    });

                table
                    .column_iter_mut()
                    .for_each(|column| column.set_cell_alignment(CellAlignment::Center));

                print!("{table}");
            } else {
                println!("file_name;entropy;conditional_entropy");

                entropies
                    .par_iter()
                    .for_each(|(file_name, (entropy, cond_entropy))| {
                        println!("{};{};{}", file_name, entropy, cond_entropy);
                    });
            }
        }

        helpers::PathType::File(file_path) => {
            let file_content: Vec<u8> = helpers::read_file(&file_path);
            let (entropy, cond_entropy): (f64, f64) = exercises::zadanie(&file_content);

            if dane.table {
                let mut table = Table::new();

                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic);

                table
                    .set_header(vec!["Entropy", "Conditional Entropy"])
                    .add_row(vec![format!("{}", entropy), format!("{}", cond_entropy)]);

                table
                    .column_iter_mut()
                    .for_each(|column| column.set_cell_alignment(CellAlignment::Center));

                print!("{table}");
            } else {
                println!("entropy;conditional_entropy");
                println!("{};{}", entropy, cond_entropy);
            }
        }
        helpers::PathType::None => panic!("Wtf Dude?!?!"),
    }
}
