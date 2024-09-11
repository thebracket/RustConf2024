use std::fs::File;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::thread::available_parallelism;
use rustc_hash::{FxHashMap, FxHasher};

#[inline(always)]
fn find_next(memory_map: &[u8], start: usize, character: char) -> usize {
    let mut i = start;
    while memory_map[i] != character as u8 {
        i += 1;
    }
    i
}

fn chunk_indices(memory_map: &[u8], num_cpus: usize) -> Vec<usize> {
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
        counter = chunk_starts_at + chunk_size + 1;
    }
    chunk_indices
}

fn ascii_slice_to_i32(buffer: &[u8]) -> i32 {
    let size = buffer.len();
    let mut negative_multiplier = 1;
    let mut accumulator = 0;
    let mut positional_mul = 10_i32.pow(size as u32 - 2);
    for i in 0 .. size {
        match buffer[i] {
            b'-' => {
                negative_multiplier = -1;
                positional_mul /= 10;
            }
            b'.' => {
                // Do nothing
            }
            48 ..= 57 => {
                // Digits
                let digit = buffer[i] as i32 - 48;
                accumulator += digit * positional_mul;
                positional_mul /= 10;
            }
            _ => panic!("Unhandled ASCII numerical symbol: {}", buffer[i]),
        }
    }
    accumulator *= negative_multiplier;
    accumulator
}

#[derive(Debug)]
struct Station {
    name: String,
    count: usize,
    min: i32,
    max: i32,
    sum: i32,
}

fn hash_station_name(station_name: &[u8]) -> u64 {
    let mut hasher = FxHasher::with_seed(0);
    hasher.write(station_name);
    hasher.finish()
}

fn pre_hash_stations() -> anyhow::Result<FxHashMap<u64, Station>> {
    let mut result = FxHashMap::default();

    let file = File::open("../data_builder/weather_stations.csv")?;
    let memory_map = unsafe { memmap::Mmap::map(&file)? };
    let mut index = 0;
    while index < memory_map.len() {
        let first_semicolon = find_next(&memory_map, index, ';');
        let station_name = &memory_map[index .. first_semicolon];

        result.insert(hash_station_name(station_name), Station {
            name: String::from_utf8_lossy(station_name).to_string(),
            count: 0,
            min: i32::MAX,
            max: i32::MIN,
            sum: 0,
        });

        let end_of_line = find_next(&memory_map, first_semicolon, '\n');
        index = end_of_line + 1;
    }
    Ok(result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    // Count available CPUs (minus 1 for the receiver)
    let num_cpus = available_parallelism()?.get() - 1;
    //let num_cpus = available_parallelism()?.get() * 2;
    //let num_cpus = available_parallelism()?.get() * 10;

    // Build the channel
    let (tx, mut rx) = mpsc::channel::<Box<Vec<(u64, i32)>>>(4096);

    // Memory map the file
    let file = File::open("../data_builder/measurements_1b.txt")?;
    let memory_map = Arc::new(unsafe { memmap::Mmap::map(&file)? });
    let chunk_indices = chunk_indices(&memory_map, num_cpus);

    // We're going to use Tokio tasks
    let mut futures = Vec::with_capacity(num_cpus + 1);

    // Spawn the calculation tasks
    for cpu in 0..num_cpus {
        // Thread-local for moving into the task
        let memory_map = memory_map.clone(); // We're only moving the pointer, not the data
        let chunk_start = chunk_indices[cpu];
        let chunk_end = if cpu == num_cpus - 1 {
            memory_map.len()
        } else {
            chunk_indices[cpu + 1]
        };
        let my_tx = tx.clone();

        let future = tokio::spawn(async move {
            let mut index = chunk_start;
            const BUFFER_SIZE: usize = 1_000;
            let mut buffer = Box::new(Vec::with_capacity(BUFFER_SIZE));
            while index < chunk_end {
                let start = index; // Where did we start?

                // Find the first string
                let i = find_next(&memory_map, start, ';');
                let station_slice = &memory_map[start .. i];

                // Find the second string
                let start = i + 1;
                let i = find_next(&memory_map, start, '\n');
                let temperature = ascii_slice_to_i32(&memory_map[start..i]);

                buffer.push((hash_station_name(station_slice), temperature));
                if buffer.len() == BUFFER_SIZE {
                    my_tx.send(buffer).await.unwrap();
                    buffer = Box::new(Vec::with_capacity(BUFFER_SIZE));
                }

                index = i + 1;
            }
            // Send the remaining buffer
            my_tx.send(buffer).await.unwrap();
        });
        futures.push(future);
    }

    // Spawn the receiver thread
    std::mem::drop(tx); // Drop the sender to signal the receiver to finish

    let _ = tokio::spawn(async move {
        // Receive the results
        let mut stations = pre_hash_stations().unwrap();
        while let Some(buffer) = rx.recv().await {
            for (hash, temperature) in buffer.iter() {
                if let Some(station) = stations.get_mut(hash) {
                    station.count += 1;
                    station.min = station.min.min(*temperature);
                    station.max = station.max.max(*temperature);
                    station.sum += *temperature;
                }
            }
        }
        //println!("Processed {} rows", counter);

        // Print the results
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        for (_, station) in stations.iter().filter(|(_, station)| station.count > 0) {
            let avg = station.sum as f32 / station.count as f32;
            writeln!(&mut lock, "{};{};{};{}", station.name, station.min as f32 / 10.0, station.max as f32 / 10.0, avg).unwrap();
        }

        println!("Elapsed: {:.4} seconds", start.elapsed().as_secs_f32());
    }).await;

    use futures::future::join_all;
    join_all(futures).await;



    Ok(())
}
