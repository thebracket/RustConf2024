use std::fs::File;
use std::io::{BufWriter, Write};
use anyhow::Context;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_distr::Normal;
use rand_xorshift::XorShiftRng;

//const ITEMS_TO_BUILD: usize = 1_000_000_000;
const ITEMS_TO_BUILD: usize = 1_000_000;

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

fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();
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

    println!("Building Measurements");
    let mut rng = XorShiftRng::from_rng(rand::thread_rng())?;
    let outfile = File::create("measurements.txt")?;
    let mut stream = BufWriter::new(outfile);
    for _ in 0 .. ITEMS_TO_BUILD {
        let station = stations.choose(&mut rng)
            .context("No weather station found")?;
        let line = format!("{};{:.1}\n", station.id, station.measurement(&mut rng));
        stream.write(line.as_bytes())?;
    }
    stream.flush()?;
    println!("Finished in {:.2} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
