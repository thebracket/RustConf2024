# Reading the Data

Armed with our `find_next` function, we can quickly make a parser that reads the bytes
and finds *slices* for station name and temperature. We can then convert those slices
into `&str`s, and parse the temperature into a `f32`.

```rust
fn read_file() -> Result<FxHashMap<String, StationReadings>> {
    let mut result: FxHashMap<String, StationReadings> = FxHashMap::default();

    // Open and memory map the file
    let file = File::open("../data_builder/measurements.txt")?;
    let memory_map = unsafe { memmap::Mmap::map(&file)? }; // It's now a big sea of bytes!

    // `index` is the cursor into the memory map - how far we've read into the file.
    // We start at the beginning.
    let mut index = 0;
    while index < memory_map.len() {
        // Store the index for where we started this iteration
        let start = index; // Where did we start?

        // Find the first string
        let i = find_next(&memory_map, start, ';');
        // Convert the bytes to an `&str` - this is the station name
        let station = std::str::from_utf8(&memory_map[start .. i])?;

        // Find the second string (the temperature)
        let start = i + 1;
        let i = find_next(&memory_map, start, '\n');
        let temperature = std::str::from_utf8(&memory_map[start..i])?;
        let temperature = temperature.parse::<f32>()?;
        index = i + 1;

        // Unchanged from the previous version
        if let Some(result) = result.get_mut(station) {
             result.max = f32::max(result.max, temperature);
             result.min = f32::min(result.min, temperature);
             result.sum += temperature;
             result.count += 1;
         } else {
             result.insert(station.to_string(), StationReadings {
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

So while we've jumped through a few hoops, it wasn't too painful.