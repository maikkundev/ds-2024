// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: May 12
// Update: May 12
// This files doesn't need to be ran

use plotters::prelude::*;
use plotters::style::full_palette::{GREY_A100, GREY_A200, GREY_A400};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results: HashMap<String, Vec<(i32, i32)>> = HashMap::new();

    let file = File::open("results.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let algorithm = parts[0].trim_end_matches(':');
        let integers: i32 = parts[2].parse()?;
        let time: i32 = parts[6].trim_end_matches("μs").parse()?;

        results
            .entry(algorithm.to_string())
            .or_default()
            .push((integers, time));
    }

    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&GREY_A200)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Sorting Algorithms Performance",
            ("sans-serif", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(32)
        .y_label_area_size(50)
        .build_cartesian_2d(100i32..1000, 0i32..700)?;

    chart
        .configure_mesh()
        .x_desc("Number of Integers")
        .y_desc("Average Time (μs)")
        .draw()?;

    let mut color_index = 0;
    for (algorithm, data) in &results {
        let color = Palette99::pick(color_index).to_rgba();
        chart
            .draw_series(LineSeries::new(data.iter().map(|(x, y)| (*x, *y)), color))?
            .label(algorithm)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        color_index += 1;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperLeft)
        .draw()?;

    Ok(())
}
