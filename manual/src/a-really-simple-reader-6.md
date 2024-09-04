# Output

Finally, a really simple function to print the results:

```rust
fn print_results(readings: Vec<Reading>) {
    for reading in readings {
        println!("{};{};{};{};", reading.station, reading.min, reading.max, reading.mean);
    }
}
```

And we're done! The final main function looks like this:

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
    println!("TOTAL:            {:.3}s", file_reader_time + hash_time + calculate_time + print_time);

    Ok(())
}
```