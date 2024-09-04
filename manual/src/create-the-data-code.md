# How Does This Work?

We're not going to workshop the data builder, but let's take a quick look at how it works. It's pretty straightforward.

## Dependencies

The data builder has a number of dependencies:

```toml
anyhow = {  workspace = true }  # Because I'm lazy about error handling!
csv = "1.3.0"                   # For reading delimited files. Very fast!
rand = "0.8.5"                  # Random number core.
rand_distr = "0.4.3"            # Distributions for `rand` - allowing Gaussian
rand_xorshift = "0.3.0"         # A faster RNG.
```

I was quite surprised when I learned that Rust's standard library didn't have a `rand` equivalent,
but I was familiar with C++ and the world's most complicated random number library (that nobody
uses)--- so I was happy to find `rand`!

`xorshift` is a fast random number generator. If we used a cryptographically safe RNG for this, we'd
be here for a while---and might heat up the building!

## Generating the Data

I started with some constants to control the data generation:

```rust
const ITEMS_TO_BUILD: usize = 1_000_000;
const FILENAME: &str = "measurements.txt";
```

Then I defined a weather station as a type:
    
```rust
struct WeatherStation {
    id: String,
    mean_temperature: f32,
}

impl WeatherStation {
    fn measurement(&self, rng: &mut XorShiftRng) -> f32 {
        let normal = Normal::new(self.mean_temperature, 10.0).unwrap();
        let m: f32 = rng.sample(normal);
        (m * 10.0).round() / 10.0
    }
}
```

The `measurement` function uses a normal (Gaussian) distribution to generate a temperature reading. It
doesn't really need to, but the original challenge used a normal distribution, so I did too.

Next, we start the `main` function. I like to return an `anyhow::Result` to make it easy to error-out from my inevitable
typos. I also start a timer:

```rust
fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();
}
```

Then we use the `csv` crate to load the weather stations:

```rust
println!("Loading Weather Stations");
let csv_reader = csv::ReaderBuilder::new()
    .comment(Some(b'#'))
    .delimiter(b';')
    .from_path("weather_stations.csv")
    .expect("Could not load weather stations");

let stations: Vec<WeatherStation> = csv_reader
    .into_records()
    .filter_map(|record| {
        let record = record.expect("Could not read record");
        let id = record.get(0).expect("Could not get id").to_string();
        let mean_temperature = record.get(1).expect("Could not get mean temperature");
        let mean_temperature = mean_temperature.parse::<f32>().ok()?;
        Some(WeatherStation {
            id,
            mean_temperature,
        })
    })
    .collect();
```

> We could have used `serde` for deserializing the weather stations, but I wanted to keep things simple.

Then we create the measurements:

```rust
println!("Building Measurements");
// Make a random number generator
let mut rng = XorShiftRng::from_rng(rand::thread_rng())?;

// Create the output file
let outfile = File::create(FILENAME)?;
// Used a buffered writer to speed things up
let mut stream = BufWriter::new(outfile);

// For each item to build...
for _ in 0 .. ITEMS_TO_BUILD {
    // Use `choose` from `rand` to randomly select a slice entry
    let station = stations.choose(&mut rng)
    // .context is lovely. It unwraps an option into an error, and lets you specify
    // the error message. It's from the `anyhow` crate.
        .context("No weather station found")?;

    // Write the station ID and the measurement to the file
    let line = format!("{};{:.1}\n", station.id, station.measurement(&mut rng));
    stream.write(line.as_bytes())?;
}
// Flush the buffer to ensure all data is written. You don't really need to do this,
// it flushes on drop automatically. I was recently bitten by some TCP server code
// that forgot to flush, so paranoia has kicked in!
stream.flush()?;

// Print the time taken
println!("Finished in {:.2} seconds", start.elapsed().as_secs_f32());
```

The data builder is pretty simple, easy to follow code. It could be faster, but we only need to run
it once or twice.