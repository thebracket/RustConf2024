# Parallel Processing

> The code for this is in `code/parallel`.

Most likely, your laptop has more than one CPU core. Each of those cores has its own cache,
and you can make them all run at once.

Concurrently accessing a giant shared byte pool sounds like a recipe for disaster. Surprisingly,
it works *really* well.

## How many cores do you have?

Last time I gave a workshop, I used the `num_cpus` crate to find out how many cores the
machine had. The standard library now has this functionality built-in:

```rust
let num_cpus = available_parallelism()?.get();
```

## Divide the Workload into Chunks

A simple approach would be:

```rust
// Split the memory map into chunks of roughly equal size
let chunk_size = memory_map.len() / num_cpus;
```

This isn't going to work, because it's unlikely that the chunks will be aligned to record
boundaries (lines). So for each chunk, we'll need to expand to the next newline character:

```rust
// BUT - chunks are probably not aligned to record boundaries/lines. So for each chunk, we'll
// need to expand to the next newline character.
let mut chunk_indices = vec![0];
let mut counter = chunk_size;
for _ in 1 .. num_cpus {
    let chunk_starts_at = find_next(
        &memory_map,
        counter,
        '\n'
    );
    chunk_indices.push(chunk_starts_at);
    counter = chunk_starts_at + chunk_size + 1;
}
```

So now we have a nice vector of indices into the memory map, defining the chunks
we can use.