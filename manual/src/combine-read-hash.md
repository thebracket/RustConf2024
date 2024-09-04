# Combine Read & Hash

> The code for this is in `code/simple_line_reader2`.

So let's combine the file reading and hashing into a single function, so we're not
allocating ourselves out of existence.

```rust
fn read_file() -> Result<HashMap<String, Vec<f32>>> {
    let mut result = HashMap::new();
    let file = File::open("../data_builder/measurements.txt")?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let station = parts.next().context("No next!")?.to_string();
        let temperature = parts.next().context("No next!")?.parse::<f32>()?;

        let entry = result.entry(station).or_insert(vec![]);
        entry.push(temperature);
    }
    Ok(result)
}
```

This function reads the file and creates the hash map in one go. It's a bit more
complicated than the previous two functions, but it's also more efficient.

Our `main` function is a little simpler, too:

```rust
fn main() -> Result<()> {
    // Setup timers
    let file_reader_time;
    let calculate_time;
    let print_time;

    // Read the file, row by row into a vector
    let stations = time_it!({
        read_file()?
    }, file_reader_time);

    // Calculate min, max and mean for each station
    let readings = time_it!({
        calculate(stations)?
    }, calculate_time);

    // Print the results
    time_it!({
        print_results(readings);
    }, print_time);


    println!("-----------------------------------------");
    println!("File & hash time: {:.3}s", file_reader_time);
    println!("Calculate time:   {:.3}s", calculate_time);
    println!("Print time:       {:.3}s", print_time);
    println!("TOTAL:            {:.3}s", file_reader_time + calculate_time + print_time);

    Ok(())
}
```
