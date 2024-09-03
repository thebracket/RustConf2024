# Combine Read & Hash

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

Here's the results (release mode):

```
File & hash time: 0.124s
Calculate time:   0.006s
Print time:       0.709s
TOTAL:            0.839s
```

So effectively the same speed. With 1 BILLION rows, it actually completes!

```
File & hash time: 117.238s
Calculate time:   1.511s
Print time:       0.841s
TOTAL:            119.590s
```

So at nearly 2 minutes, we're not going to be winning any prizes for speed --- but at least this
version finished!

It's often said that Rust's hash function is slow (it is cryptographically safe!).
Let's replace it!