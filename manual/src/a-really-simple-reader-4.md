# Store Readings in Buckets

```rust
fn hash_file(readings: &[RawReading]) -> Result<HashMap<String, Vec<f32>>> {
    let result = readings.iter().fold(HashMap::new(), |mut acc, reading| {
        acc.entry(reading.station.clone()).or_insert(vec![]).push(reading.temperature);
        acc
    });
    Ok(result)
}
```

We iterate through the readings, and "fold" the results into a `HashMap`. Folding is
a simple way to accumulate results --- it's just the same as making a for loop with
a shared variable.

`entry` is a handy method on `HashMap` that returns a reference to the value if it
exists, or inserts a new value if it doesn't. This is a common pattern in Rust, and
it's a good way to avoid double-lookups.

Our `main` function now looks like this:

```rust
fn main() -> anyhow::Result<()> {
    // Setup timers
    let file_reader_time;
    let hash_time;

    // Read the file, row by row into a vector
    let rows = time_it!({
        read_file()?
    }, file_reader_time);

    // Hash the rows by station
    let stations = time_it!({
        hash_file(&rows)?
    }, hash_time);

    println!("-----------------------------------------");
    println!("File reader time: {:.3}s", file_reader_time);
    println!("Hash time:        {:.3}s", hash_time);
    
    Ok(())
}
```