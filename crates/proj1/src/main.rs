// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: March 9
// Update: May 12
// cargo run --bin ds-2024

pub mod record;

use glob::glob;
use std::process::Command;

fn main() {
    for entry in glob("./*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                if file_name != "main" && file_name != "plottings" {
                    Command::new("cargo")
                        .args(["run", "--bin", file_name, "--release"])
                        .status()
                        .expect("Failed to execute command");
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
