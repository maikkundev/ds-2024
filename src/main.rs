// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
//

use std::process::Command;

use glob::glob;

fn main() {
    for file in glob("./*").expect("Failed to read glob pattern") {
        Command::new("cargo")
            .args([
                "--bin",
                file.unwrap().file_name().unwrap().to_str().unwrap(),
            ])
            .spawn()
            .expect("Failed to spawn command");
    }
}
