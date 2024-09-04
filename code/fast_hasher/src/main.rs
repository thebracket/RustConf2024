use std::fs::File;
use std::io::BufRead;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

fn read_file() -> Result<FxHashMap<String, Vec<f32>>> {
    let mut result = FxHashMap::default();
    let file = File::open("../data_builder/measurements.txt")?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;

        let entry = result.entry(station).or_insert(vec![]);
        entry.push(temperature);
    }
    Ok(result)
}

struct Reading {
    station: String,
    min: f32,
    max: f32,
    mean: f32,
}

fn calculate(readings: FxHashMap<String, Vec<f32>>) -> Result<Vec<Reading>> {
    let mut result = vec![];
    readings.into_iter().for_each(|(station, readings)| {
        let min = readings.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = readings.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let sum = readings.iter().sum::<f32>();
        let mean = sum / readings.len() as f32;
        result.push(Reading { station, min: *min, max: *max, mean });
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
