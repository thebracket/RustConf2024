# Build an Accumulator

Let's start by defining the data we *do* need per station:

```rust
#[derive(Default)]
struct StationReadings {
    min: f32,
    max: f32,
    sum: f32,
    count: usize,
}
```

We have the minimum, maximum, sum and count of readings. We'll use `f32` for the readings, as that's what we're reading from the file. We'll use `usize` for the count, as that's what we're accumulating.

Now we can replace the `read_file` function with one that accumulates the data instead of storing every entry:

```rust
fn read_file() -> Result<FxHashMap<String, StationReadings>> {
    // Set up the result to store the readings for each station
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();
    
    // Read the file as before
    let file = File::open("../data_builder/measurements.txt")?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;

        // If the station is already in the result, update the min/max/sum/count
        if let Some(result) = result.get_mut(&station) {
            result.max = f32::max(result.max, temperature);
            result.min = f32::min(result.min, temperature);
            result.sum += temperature;
            result.count += 1;
        } else {
            // Otherwise, insert a new entry
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
```

The `calculate` function is now simpler:

```rust
fn calculate(readings: FxHashMap<String, StationReadings>) -> Result<Vec<Reading>> {
    let mut result = vec![];
    readings.into_iter().for_each(|(station, readings)| {
        let mean = readings.sum / readings.count as f32;
        result.push(Reading { station, min: readings.min, max: readings.max, mean });
    });
    Ok(result)
}
```

We're no longer allocating space for a billion `f32` values, nor are we copying them around.