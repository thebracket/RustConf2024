# Replace the Hash

> The code for this is in `code/fast_hasher`.

## Background

Rust's `HashMap` uses a hash function that is *very* unlikely to give you collisions. (A collision is when two
different keys hash to the same value.) This is great for most applications, but it comes at a price: it's not
the fastest hash function around.

Every time you insert a key into the `HashMap`, the input string is "hashed"---fed into a `Hasher` that
produces a 64-bit hash value. This is then used to determine where in the `HashMap` the key should be stored.

Every time you look up a key in the `HashMap`, the input string is hashed again, and the `HashMap` uses that
hash to find the key.

So for a billion rows, you are hashing a billion times on input. Every search hashes the key again. That's a *lot*
of hashing!

## FxHashMap to the Rescue

It's pretty common wisdom nowadays to use `FxHashMap` when cyrpoto-quality hashing isn't needed. `FxHashMap` is
a "fast" hash map that uses a simpler hash function. It's not as good at avoiding collisions, but it's *much*
faster. It's used by `rustc`, and lots of other projects.

First, we add a dependency:

```toml
rustc-hash = "2.0.0"
```

And replace all of the `HashMap` with `FxHashMap`. We have to replace `HashMap::new()` with `FxHashMap::default()`,
because there is no `new()` method for `FxHashMap`.

```rust
// Change the return type
fn read_file() -> Result<FxHashMap<String, Vec<f32>>> {
    // Change the hash map type
    let mut result = FxHashMap::default();
    // EVERYTHING else is the same!
    let file = File::open("../data_builder/measurements_1b.txt")?;
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
