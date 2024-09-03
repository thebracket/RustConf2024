use std::fs::File;
use std::thread;
use std::thread::available_parallelism;
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
    let num_cpus = available_parallelism()?.get();
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();

    let file = File::open("../data_builder/measurements_1b.txt")?;
    let memory_map = unsafe { memmap::Mmap::map(&file)? }; // It's now a big sea of bytes!

    // Split the memory map into chunks of roughly equal size
    let chunk_size = memory_map.len() / num_cpus;

    // BUT - chunks are probably not aligned to record boundaries/lines. So for each chunk, we'll
    // need to expand to the next newline character.
    let mut chunk_indices = vec![0];
    let mut counter = chunk_size;
    for _ in 1 .. num_cpus {
        let chunk_starts_at = find_next(
            &memory_map,
            counter,
            '\n'
        );
        chunk_indices.push(chunk_starts_at);
        counter = chunk_starts_at + chunk_size;
    }

    // Now we can spawn threads to process each chunk. We'll use scoped threads to make
    // it easier to manage the lifetimes of the threads.
    thread::scope(|scope| {
        let mut handles = vec![];
        for cpu in 0 .. num_cpus {
            // Start by acquiring our own copy of variables to move.
            let memory_map = &memory_map; // We're only moving the pointer, not the data
            let chunk_start = chunk_indices[cpu];
            let chunk_end = if cpu == num_cpus - 1 {
                memory_map.len()
            } else {
                chunk_indices[cpu + 1]
            };
            let handle = scope.spawn(move || {
                let mut local_result: FxHashMap<String, StationReadings> = FxHashMap::default();
                let mut index = chunk_start;
                while index < chunk_end {
                    let start = index; // Where did we start?

                    // Find the first string
                    let i = find_next(&memory_map, start, ';');
                    let station = std::str::from_utf8(&memory_map[start .. i]).unwrap();

                    // Find the second string
                    let start = i + 1;
                    let i = find_next(&memory_map, start, '\n');
                    let temperature = std::str::from_utf8(&memory_map[start..i]).unwrap();
                    let temperature = temperature.parse::<f32>().unwrap();
                    index = i + 1;

                    if let Some(result) = local_result.get_mut(station) {
                        result.max = f32::max(result.max, temperature);
                        result.min = f32::min(result.min, temperature);
                        result.sum += temperature;
                        result.count += 1;
                    } else {
                        local_result.insert(station.to_string(), StationReadings {
                            min: temperature,
                            max: temperature,
                            sum: temperature,
                            count: 1,
                        });
                    }
                }

                local_result
            }); // End thread
            handles.push(handle);
        }

        for handle in handles {
            let local_result = handle.join().unwrap();
            for (station, readings) in local_result {
                if let Some(result) = result.get_mut(&station) {
                    result.min = f32::min(result.min, readings.min);
                    result.max = f32::max(result.max, readings.max);
                    result.sum += readings.sum;
                    result.count += readings.count;
                } else {
                    result.insert(station, readings);
                }
            }
        }
    });

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
