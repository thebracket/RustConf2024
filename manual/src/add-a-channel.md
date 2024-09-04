# Add a Channel

> See `code/channelize` for the code.

Let's pull together every dirty trick I could think of that didn't involve writing assembly, trying to use SIMD instructions,
or drive me completely crazy!

## Pre-hashing Station Names

The first thing we're going to do is create a function that pre-hashes station names. We know that stations
will appear in the input exactly the same as the weather stations CSV file. So we can take advantage of that
to pre-hash the station names.

```rust
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
```

Notice that we're building an accumulator for each station, keyed by the
station *hash*.

## Adding a Channel

We're going to dedicate one core to accumulating data, and the rest to reading the file. We'll use a channel to
communicate between the two.

First, we reduce the number of cores we're using:

```rust
// Count available CPUs (minus 1 for the receiver)
let num_cpus = available_parallelism()?.get() - 1;
```

Then we build a channel:

```rust
// Build the channel
let (tx, rx) = mpsc::channel::<Box<Vec<(u64, i32)>>>();
```

The `tx` end of the channel is used by the reader to send data to the accumulator. The `rx` end is used by the accumulator to receive data.

So why the `Vec`? Sending results one at a time used a LOT of memory, and was slow. Sending batches of results was faster, and used less memory.

So why the `Box`? My testing showed that it is slightly faster, because instead of copying the `Vec` we're just copying
a pointer to the `Vec`.

## The Reader

The reader is pretty much the same as what we had before, but we've added a batched channel submission:

```rust
scope.spawn(move || {
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
            my_tx.send(buffer).unwrap();
            buffer = Box::new(Vec::with_capacity(BUFFER_SIZE));
        }

        index = i + 1;
    }
    // Send the remaining buffer
    my_tx.send(buffer).unwrap();
});
```

## Garbage Disposal

Notice that we're also cloning `tx` into each thread - and dropping the original after we've spawned the threads.
`Sender` works like `Arc`: it's reference counted. This guarantees that when all threads are done, the channel
will close. We're using this to our advantage.

## The Receiver

Inside the thread scope, we spawn one more thread:

```rust
// Spawn the receiver thread
scope.spawn(move || {
    // Receive the results
    let mut stations = pre_hash_stations().unwrap();
    while let Ok(buffer) = rx.recv() {
        for (hash, temperature) in buffer.iter() {
            if let Some(station) = stations.get_mut(hash) {
                station.count += 1;
                station.min = station.min.min(*temperature);
                station.max = station.max.max(*temperature);
                station.sum += *temperature;
            }
        }
    }
    // Continues..
```

This will keep receiving results until the channel is closed---which will happen when there are no more
results to receive and all the senders are dropped when their threads finish. Notice that we're just sending
the reading and the hashed station name - a `u64` for the hash, and an `i32` for the temperature. That's where
pre-hashing comes in - we already have a hashmap of stations, so we can just look up the station by hash. No
allocation once the initial creation is done.

## Printing the Results

I mentioned that we could optimize `writeln!`. `println!` and similar take a Mutex lock to `stdout`. This is
preferable when you are writing threaded demos, you don't wind up with a soup of overlapping strings. It's
not so good for performance. Instead, we lock once and write multiple times:

```rust
// Print the results
use std::io::Write;
let stdout = std::io::stdout();
let mut lock = stdout.lock();
for (_, station) in stations.iter().filter(|(_, station)| station.count > 0) {
    let avg = station.sum as f32 / station.count as f32;
    writeln!(&mut lock, "{};{};{};{}", station.name, station.min as f32 / 10.0, station.max as f32 / 10.0, avg).unwrap();
}
```

## The Results

So how did we do? We've thrown the kitchen sink at the task!

With 1 million rows:

```
Elapsed: 0.7354 seconds
```

With 1 billion rows:

```
Elapsed: 12.6407 seconds
```