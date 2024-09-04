use std::fs::File;
use std::hash::Hasher;
use std::hint::black_box;
use std::thread;
use std::thread::available_parallelism;
use rustc_hash::{FxHashSet, FxHasher};

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

fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    // Count available CPUs
    let num_cpus = available_parallelism()?.get();

    // Memory map the file
    let file = File::open("../data_builder/measurements_1b.txt")?;
    let memory_map = unsafe { memmap::Mmap::map(&file)? };
    let chunk_indices = chunk_indices(&memory_map, num_cpus);

    // Scoped threads
    thread::scope(|scope| {
        for cpu in 0..num_cpus {
            // Thread-local for moving into the thread
            let memory_map = &memory_map; // We're only moving the pointer, not the data
            let chunk_start = chunk_indices[cpu];
            let chunk_end = if cpu == num_cpus - 1 {
                memory_map.len()
            } else {
                chunk_indices[cpu + 1]
            };

            scope.spawn(move || {
                let mut index = chunk_start;
                let mut name_set = FxHashSet::default();
                let mut counter = 0;
                while index < chunk_end {
                    let start = index; // Where did we start?

                    // Find the first string
                    let i = find_next(&memory_map, start, ';');
                    let station_slice = &memory_map[start .. i];

                    // Hash the station name
                    let mut hasher = FxHasher::with_seed(0);
                    hasher.write(station_slice);
                    let hash = hasher.finish();
                    name_set.insert(hash);
                    counter += 1;

                    // Find the second string
                    let start = i + 1;
                    let i = find_next(&memory_map, start, '\n');
                    black_box(&memory_map[start..i]);

                    index = i + 1;
                }

                println!("CPU {} processed {} stations, {counter} rows", cpu, name_set.len());
            });
        }
    });

    println!("Elapsed: {:.4} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
