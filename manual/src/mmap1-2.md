# Finding Data in a Sea of Bytes

Now you have your file as a single `&[u8]` - a big sea of bytes. You could use the
various `Cursor` types to read through it, but that's overkill---and starts to get
a little messier when we want to parallelize.

So let's create a simple function to read through a haystack of bytes, and find the
next occurrence of the needle:

```rust
fn find_next(memory_map: &[u8], start: usize, character: char) -> usize {
    let mut i = start;
    while memory_map[i] != character as u8 {
        i += 1;
    }
    i
}
```

This is deliberately straightforward. We check each byte in turn to see if
it's the character we're looking for. If it is, we return the index. If not,
we keep going.

> Note that we're not doing a bounds-check here. The program will panic --- rather than segfault --- if
> you read past the end of the memory map. We're doing this for performance, and to keep the class
> time manageable. In a production system, you'd want to add bounds-checking.