use std::fs::File;
use std::io::BufRead;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

#[derive(Default)]
struct StationReadings {
    min: f32,
    max: f32,
    sum: f32,
    count: usize,
}

fn read_file() -> Result<FxHashMap<String, StationReadings>> {
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();
    let file = File::open("../data_builder/measurements_1b.txt")?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;

        if let Some(result) = result.get_mut(&station) {
            result.max = f32::max(result.max, temperature);
            result.min = f32::min(result.min, temperature);
            result.sum += temperature;
            result.count += 1;
        } else {
            result.insert(station.clone(), StationReadings {
                min: temperature,
                max: temperature,
                sum: temperature,
                count: 1,
            });
        }
    }
    Ok(result)
}

struct Reading {
    station: String,
    min: f32,
    max: f32,
    mean: f32,
}

fn calculate(readings: FxHashMap<String, StationReadings>) -> Result<Vec<Reading>> {
    let mut result = vec![];
    readings.into_iter().for_each(|(station, readings)| {
        let mean = readings.sum / readings.count as f32;
        result.push(Reading { station, min: readings.min, max: readings.max, mean });
    });
    Ok(result)
}

fn print_results(readings: Vec<Reading>) {
    for reading in readings {
        println!("{};{};{};{};", reading.station, reading.min, reading.max, reading.mean);
    }
}

macro_rules! time_it {
    ($block:block, $timer:expr) => {
        {
            let start = std::time::Instant::now();
            let result = $block;
            let elapsed = start.elapsed().as_secs_f32();
            $timer = elapsed;
            result
        }
    };
}

fn main() -> Result<()> {
    // Setup timers
    let file_reader_time;
    let calculate_time;
    let print_time;

    // Read the file, row by row into a vector
    let stations = time_it!({
        read_file()?
    }, file_reader_time);

    // Calculate min, max and mean for each station
    let readings = time_it!({
        calculate(stations)?
    }, calculate_time);

    // Print the results
    time_it!({
        print_results(readings);
    }, print_time);


    println!("-----------------------------------------");
    println!("File & hash time: {:.3}s", file_reader_time);
    println!("Calculate time:   {:.3}s", calculate_time);
    println!("Print time:       {:.3}s", print_time);
    println!("TOTAL:            {:.3}s", file_reader_time + calculate_time + print_time);

    Ok(())
}
