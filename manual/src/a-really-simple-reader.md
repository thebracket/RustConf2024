# A Really Simple Reader

So let's start with a really simple, step-by-step reader.

```rust
struct RawReading {
    station: String,
    temperature: f32,
}

fn read_file() -> Result<Vec<RawReading>> {
    let file = File::open("../data_builder/measurements.txt")?;
    let reader = std::io::BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;
        result.push(RawReading { station, temperature });
    }
    Ok(result)
}
```

```rust
fn hash_file(readings: &[RawReading]) -> Result<HashMap<String, Vec<f32>>> {
    let result = readings.iter().fold(HashMap::new(), |mut acc, reading| {
        acc.entry(reading.station.clone()).or_insert(vec![]).push(reading.temperature);
        acc
    });
    Ok(result)
}
```

```rust
struct Reading {
    station: String,
    min: f32,
    max: f32,
    mean: f32,
}

fn calculate(readings: HashMap<String, Vec<f32>>) -> Result<Vec<Reading>> {
    let mut result = vec![];
    readings.into_iter().for_each(|(station, mut readings)| {
        let min = readings.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = readings.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let sum = readings.iter().sum::<f32>();
        let mean = sum / readings.len() as f32;
        result.push(Reading { station, min: *min, max: *max, mean });
    });
    Ok(result)
}
```

```rust
fn print_results(readings: Vec<Reading>) {
    for reading in readings {
        println!("{};{};{};{};", reading.station, reading.min, reading.max, reading.mean);
    }
}
```

A handy macro!

```rust
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
```

And the main function:

```rust
fn main() -> Result<()> {
    // Setup timers
    let file_reader_time;
    let hash_time;
    let calculate_time;
    let print_time;

    // Read the file, row by row into a vector
    let rows = time_it!({
        read_file()?
    }, file_reader_time);

    // Hash the rows by station
    let stations = time_it!({
        hash_file(&rows)?
    }, hash_time);

    // Calculate min, max and mean for each station
    let readings = time_it!({
        calculate(stations)?
    }, calculate_time);

    // Print the results
    time_it!({
        print_results(readings);
    }, print_time);


    println!("-----------------------------------------");
    println!("File reader time: {:.3}s", file_reader_time);
    println!("Hash time:        {:.3}s", hash_time);
    println!("Calculate time:   {:.3}s", calculate_time);
    println!("Print time:       {:.3}s", print_time);

    Ok(())
}
```

## Oh Dear!

Running this with a million rows works for my system. The timings aren't even terrible:

```
File reader time: 0.094s
Hash time:        0.070s
Calculate time:   0.006s
Print time:       0.692s
TOTAL:            0.862s
```

But when I ran it for a BILLION rows, my PC crashed. I completely exhausted my RAM and swap
file, and the system became unresponsive. I had to hit the power button. Oops.

What went wrong? There's a LOT we can improve, but let's start with the file reader. We're
reading every single reading into a new `String` and `f32`, and then pushing it into a `Vec`.
We then read these and get rid of them in the next step - so we're allocating 1,000,000,000 new
strings and `f32` values, only to throw them away. That's a lot of memory churn---and a lot of
memory to allocate.

Let's try and do better.