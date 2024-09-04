# How fast CAN we go?

Let's refactor (see `blazing_fast`) and get an idea of how I/O bound we actually are. We'll walk through
the `code/blazing_fast` project. There's a lot of changes (including it doesn't actually solve the challenge):

* Pretty much everything is in `main` now.
* We're just parsing the memory mapped file and accessing slices. We're not doing anything with the data.

## What's up with `blackbox`?

The Rust compiler is *really good* at optimizing away code that doesn't do anything. That's great for
production code. It not at all good for benchmarking. `blackbox` tells the compiler that it can't discard
the result of the function, so it has to actually run it.

## Let's read the whole file

Surprisingly, we're really not I/O bound. For 1 million rows we take 0.0160 seconds. Or 1 billion rows: 14.38 seconds!
