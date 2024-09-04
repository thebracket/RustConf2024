# Create the Data

In your cloned repo, there's a project: `code/data_builder`. This project generates the data we'll use for the challenge.

> I don't want you to have to sit and wait while 1,000,000,000 rows are processed. We only have a 3 hour workshop
> slot! So the builder will make 1,000,000 rows by default.

Go ahead and run the project:

```bash
cd code/data_builder
cargo run --release
# Loading Weather Stations
# Building Measurements
# Finished in 0.14 seconds
```

You should see a file appear named `measurements.txt`. It's about 15 megabytes in size.

---

## If You'd Like to Try for a Billion!

> Warning: the input file is 15 Gigabytes in size and takes a while to generate!

Open `src/main.rs`, and switch the comments around:

```rust
//const ITEMS_TO_BUILD: usize = 1_000_000_000;
//const FILENAME: &str = "measurements_1b.txt";

const ITEMS_TO_BUILD: usize = 1_000_000;
const FILENAME: &str = "measurements.txt";
```

Run it again, and it'll take a bit longer. Quite a lot longer (124 seconds on my workstation)! The resulting file is
15 Gigabytes in size (which is why it's not in the repo).