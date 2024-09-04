# Parsing Numbers from ASCII

We've been reading bytes into a string, converting that string into an `f32` and then using the result.
That's not exactly super-fast --- we're doing a lot of work to convert the ASCII into a number.

## Can we parse the number directly from the ASCII?

There are various crates that do a great job of parsing data (or you can drop down to `libc`). Instead, I hacked
together a simple parser:

```rust
fn ascii_slice_to_i32(buffer: &[u8]) -> i32 {
    let size = buffer.len();
    let mut negative_multiplier = 1;
    let mut accumulator = 0;
    let mut positional_mul = 10_i32.pow(size as u32 - 2);
    for i in 0 .. size {
        match buffer[i] {
            b'-' => {
                negative_multiplier = -1;
                positional_mul /= 10;
            }
            b'.' => {
                // Do nothing
            }
            48 ..= 57 => {
                // Digits
                let digit = buffer[i] as i32 - 48;
                accumulator += digit * positional_mul;
                positional_mul /= 10;
            }
            _ => panic!("Unhandled ASCII numerical symbol: {}", buffer[i]),
        }
    }
    accumulator *= negative_multiplier;
    accumulator
}
```

After a bit of testing, it works pretty well. It returns an `i32`---but all of our numbers have exactly one
decimal place, so we can multiply by 10 to get an `f32`.

## Trying it Out

We're still not actually solving the challenge, but now we can get an idea of the timings. Running `code/blazing_fast3`
for 1 million rows:

```
CPU 4 processed 28468 stations, 49991 rows
CPU 2 processed 28357 stations, 50057 rows
CPU 0 processed 28306 stations, 49952 rows
CPU 8 processed 28317 stations, 49969 rows
CPU 6 processed 28399 stations, 49995 rows
CPU 11 processed 28304 stations, 50056 rows
CPU 7 processed 28410 stations, 50001 rows
CPU 10 processed 28314 stations, 49961 rows
CPU 1 processed 28308 stations, 49947 rows
CPU 14 processed 28364 stations, 50112 rows
CPU 13 processed 28292 stations, 49912 rows
CPU 12 processed 28338 stations, 49982 rows
CPU 9 processed 28465 stations, 50012 rows
CPU 15 processed 28346 stations, 50009 rows
CPU 5 processed 28367 stations, 50069 rows
CPU 16 processed 28386 stations, 49988 rows
CPU 18 processed 28408 stations, 50011 rows
CPU 3 processed 28346 stations, 50027 rows
CPU 17 processed 28333 stations, 49916 rows
CPU 19 processed 28372 stations, 50033 rows
Elapsed: 0.0058 seconds
```

That's really fast. How about a billion rows?

```
CPU 19 processed 41343 stations, 50000624 rows
CPU 16 processed 41343 stations, 49999319 rows
CPU 11 processed 41343 stations, 49997476 rows
CPU 12 processed 41343 stations, 49998829 rows
CPU 10 processed 41343 stations, 50002812 rows
CPU 13 processed 41343 stations, 49996774 rows
CPU 17 processed 41343 stations, 50002729 rows
CPU 14 processed 41343 stations, 50002262 rows
CPU 3 processed 41343 stations, 50000434 rows
CPU 2 processed 41343 stations, 50002224 rows
CPU 0 processed 41342 stations, 49998847 rows
CPU 7 processed 41343 stations, 50000450 rows
CPU 5 processed 41343 stations, 49999925 rows
CPU 15 processed 41343 stations, 50000573 rows
CPU 9 processed 41343 stations, 49997645 rows
CPU 8 processed 41343 stations, 49999656 rows
CPU 18 processed 41343 stations, 50001427 rows
CPU 6 processed 41343 stations, 50000718 rows
CPU 1 processed 41343 stations, 49996369 rows
CPU 4 processed 41343 stations, 50000907 rows
Elapsed: 1.9767 seconds
```

So we've lost a little bit of speed, but parsing all the numbers as slice iteration is still *really* fast.

There's a few reasons why this is so fast:

* We're not allocating memory.
* We're not doing expensive ASCII to UTF-8 conversions.
* Each core is working on a separate chunk of data. It's only reading forward, which is the ideal case for the core's cache.
* We're not actually doing the math yet....

We'd like to retain cache coherency, so we need a way to send the data out of the processing thread. Let's add a channel.