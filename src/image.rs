extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate plotters;

use chrono::{Local, DateTime};
use plotters::prelude::*;
use std::fs::File;
use std::io::BufReader;
use super::log;

pub fn generate(data_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // データを保持するJSONファイルを開く
    let file = File::open(data_file_path)?;
    let reader = BufReader::new(file);

    // JSONファイルからVec<Log>を読み取る
    let logs: Vec<log::Log> = serde_json::from_reader(reader)?;

    let xs: Vec<DateTime<Local>> = logs.iter().map(|x| x.time).collect();
    let ys: Vec<i64> = logs.iter().map(|x| x.status.co2ppm).collect();

    // グラフを設定する
    let root = BitMapBackend::new("output.png", (1200, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("CO2 ppm", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            *xs.first().unwrap()..*xs.last().unwrap(),
            *ys.iter().min().unwrap()..*ys.iter().max().unwrap(),
        )?;

    chart.configure_mesh()
    .x_label_formatter(&|x: &DateTime<Local>| x.format("%Y/%m/%d %H:%M").to_string())
    .draw()?;

    let line_series = LineSeries::new(
        xs.iter()
          .zip(ys.iter())
          .map(|(x, y)| (*x, *y)),
        &RED
       );
    chart.draw_series(line_series)?;

    Ok(())
}