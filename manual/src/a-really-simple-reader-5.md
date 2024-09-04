# Calculate Each Station

For each stored station, we find the minimum, maximum and sum of the readings. The mean average is
just the sum divided by the number of readings.

```rust
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

So now our main function looks like this:

```rust
fn main() -> anyhow::Result<()> {
    // Setup timers
    let file_reader_time;
    let hash_time;
    let calculate_time;

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
    println!("Calculate time:   {:.3}s", calculate_time);
    
    Ok(())
}
```