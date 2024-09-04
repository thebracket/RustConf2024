# Readings One by One

We can start with a simple structure to hold the readings:

```rust
struct RawReading {
    station: String,
    temperature: f32,
}
```

Now let's make a function that reads them one at a time:

```rust
use std::fs::File;
use std::io::BufRead;

fn read_file() -> Result<Vec<RawReading>> {
    // Open the file
    let file = File::open("../data_builder/measurements.txt")?;
    
    // Use a buffered reader to read line by line
    let reader = std::io::BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines() {
        // Read the line and split on the semicolon
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;
        // Store the reading
        result.push(RawReading { station, temperature });
    }
    Ok(result)
}
```

This is very naive, but it's a good starting point. Your `main` function can call it as follows:

```rust
fn main() -> anyhow::Result<()> {
    // Setup timers
    let file_reader_time;

    // Read the file, row by row into a vector
    let rows = time_it!({
        read_file()?
    }, file_reader_time);

    println!("-----------------------------------------");
    println!("File reader time: {:.3}s", file_reader_time);
    
    Ok(())
}
```