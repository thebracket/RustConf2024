# Accumulate

> The code for this is in `code/accumulate`.

If you look at the code we have so far, we're:

1. Reading the file, one row at a time.
2. Each row is hashed to a station.
3. We push the reading onto the vector for that station.
4. We calculate the min/max/mean for each station at the end.

Since vector's grow as you add to them, we're doing a ton of memory allocation. We're also trashing our CPU
cache by jumping around all over the place: each vector consists of a pointer to its capacity and its size. So
every single reading is finding the vector in the hash table, following the pointer, and then appending to
the vector's buffer. On top of that, every time we fill up the vector, Rust helpfully reallocates it to double
its size, copying all the data over.

And here's the kicker: to calculate a min/max/mean, we don't need to store every reading. We just need to
accumulate the minimum, maximum, sum and count of values.

In other words, we're doing a *lot* more work than we need to.
