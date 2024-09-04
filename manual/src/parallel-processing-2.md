# Threaded Reader

Now that we have chunks of data, we can spawn some threads. We're going to use `std::thread::scope` rather than
`std::thread` to make it easy to access/capture local data.

We'll start by acquiring our own copy of variables to move. We're only moving the pointer, not the data:

```rust
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
        // (continues)
```

> You might be tempted to use `Arc`. You could, but it's not necessary.

First, we make a vector to hold thread join handles. Scoped threads automatically join, but we want
to receive results from the threads - so we need to keep track of the join handles.

Then we loop through each available CPU core and make our own thread-local accessor: a reference (pointer) to
the memory map---we aren't duplicating it. We store the start and end indices for the thread, being careful
not to list past the end of the map for the last thread.

## The Thread

Inside each thread, we build our own accumulator:

```rust
let handle = scope.spawn(move || {
    // Build our own local data set
    let mut local_result: FxHashMap<String, StationReadings> = FxHashMap::default();

    // Start reading the memory map at the beginning of the chunk
    let mut index = chunk_start;
    while index < chunk_end {
        // The rest is unchanged, other than writing to the LOCAL result
        // ...
    }

    local_result
}); // End thread
handles.push(handle);
```

Still inside the scope, but after spawning - we wait for results and combine them:

```rust
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
```

## The Reader

The `read_file` function is now rather large:

```rust
fn read_file() -> Result<FxHashMap<String, StationReadings>> {
    let num_cpus = available_parallelism()?.get();
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();

    let file = File::open("../data_builder/measurements.txt")?;
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
        counter = chunk_starts_at + chunk_size + 1;
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
```