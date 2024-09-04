# Bytes vs Characters

Rust `char` isn't a single ASCII byte. It's a Unicode character, which can be several bytes. So when we load our `u8` into
printable strings, Rust is helpfully doing all the checks required to make sure that we're not printing invalid UTF-8. That's
usually *really* helpful - in this case, it's a performance hit. In the worst case, `from_str` can involve a copy
and a check for every byte in the string!

From an ASCII - pure `u8` slice - point of view, these are equivalent:

```
[77, 111, 117, 110, 116, 32, 86, 101, 114, 110, 111, 110]
Mount Vernon
```

Currently, we're reading the ASCII into a string and then hashing the string. That's not exactly "zero copy"---we're
invoking overhead on each string read.

## Can we hash on a `u8` slice? Yes we can!

We can't just add `u8` slices into a `HashSet`: the slice is a pointer (plus length), so we'll have one
hash per pointer rather than one per string. So instead, we'll use `FxHasher` to hash the contents of the slice
and store the resulting hash.

The following function will hash a `u8` slice:

```rust
fn hash_station_name(station_name: &[u8]) -> u64 {
    let mut hasher = FxHasher::with_seed(0);
    hasher.write(station_name);
    hasher.finish()
}
```

## Trying it Out

Let's open up `code/blazing_fast2`. It still doesn't solve the problem, but it adds some functionality. We hash
each station name, and add it to a set (proving that we read it).

Wow, that's really fast. 1 million rows:

```
CPU 5 processed 28439 stations, 50077 rows
CPU 7 processed 28484 stations, 49992 rows
CPU 8 processed 28446 stations, 50002 rows
CPU 9 processed 28318 stations, 49993 rows
CPU 11 processed 28486 stations, 50019 rows
CPU 1 processed 28271 stations, 50011 rows
CPU 4 processed 28284 stations, 50156 rows
CPU 2 processed 28387 stations, 50037 rows
CPU 3 processed 28358 stations, 50075 rows
CPU 13 processed 28370 stations, 49998 rows
CPU 6 processed 28356 stations, 50000 rows
CPU 12 processed 28254 stations, 49938 rows
CPU 16 processed 28303 stations, 49947 rows
CPU 17 processed 28276 stations, 49969 rows
CPU 14 processed 28268 stations, 49892 rows
CPU 15 processed 28203 stations, 49951 rows
CPU 18 processed 28364 stations, 50029 rows
CPU 19 processed 28430 stations, 50054 rows
CPU 0 processed 28239 stations, 49896 rows
CPU 10 processed 28238 stations, 49964 rows
Elapsed: 0.0361 seconds
```

1 billion rows:

```
CPU 14 processed 41343 stations, 50000042 rows
CPU 8 processed 41343 stations, 50002282 rows
CPU 4 processed 41343 stations, 50000446 rows
CPU 0 processed 41342 stations, 49998459 rows
CPU 12 processed 41343 stations, 49999133 rows
CPU 18 processed 41343 stations, 50001362 rows
CPU 6 processed 41343 stations, 50000972 rows
CPU 9 processed 41343 stations, 49998751 rows
CPU 15 processed 41343 stations, 49997740 rows
CPU 2 processed 41343 stations, 50000465 rows
CPU 17 processed 41343 stations, 49999767 rows
CPU 7 processed 41343 stations, 49999852 rows
CPU 19 processed 41343 stations, 50000780 rows
CPU 13 processed 41343 stations, 50001311 rows
CPU 5 processed 41343 stations, 50000166 rows
CPU 10 processed 41343 stations, 49999500 rows
CPU 1 processed 41343 stations, 50002782 rows
CPU 16 processed 41343 stations, 50001350 rows
CPU 11 processed 41343 stations, 49996459 rows
CPU 3 processed 41343 stations, 49998381 rows
Elapsed: 1.8955 seconds

```

1.8955 seconds to uniquely hash all the readings in a billion rows. 
As my friend Robert says, "now we're cooking with gas".

So now we're uniquely identifying each station as a `u64` hash. We still need to obtain the temperature reading.

`blazing_fast3`