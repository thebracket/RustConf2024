use std::fs::File;
use anyhow::Result;
use rustc_hash::FxHashMap;

#[derive(Default)]
struct StationReadings {
    min: f32,
    max: f32,
    sum: f32,
    count: usize,
}

fn find_next(memory_map: &[u8], start: usize, character: char) -> usize {
    let mut i = start;
    while memory_map[i] != character as u8 {
        i += 1;
    }
    i
}

fn read_file() -> Result<FxHashMap<String, StationReadings>> {
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();

    let file = File::open("../data_builder/measurements_1b.txt")?;
    let memory_map = unsafe { memmap::Mmap::map(&file)? }; // It's now a big sea of bytes!

    let mut index = 0;
    while index < memory_map.len() {
        let start = index; // Where did we start?

        // Find the first string
        let i = find_next(&memory_map, start, ';');
        let station = std::str::from_utf8(&memory_map[start .. i])?;

        // Find the second string
        let start = i + 1;
        let i = find_next(&memory_map, start, '\n');
        let temperature = std::str::from_utf8(&memory_map[start..i])?;
        let temperature = temperature.parse::<f32>()?;
        index = i + 1;

        if let Some(result) = result.get_mut(station) {
             result.max = f32::max(result.max, temperature);
             result.min = f32::min(result.min, temperature);
             result.sum += temperature;
             result.count += 1;
         } else {
             result.insert(station.to_string(), StationReadings {
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
