# Wait, it's not that trappy?

If you have a look at `code/tokio_challenge`, there's a Tokio-ised async version of the billion rows challenge.

It's not that bad. We had to add `tokio` and `futures` as dependencies, replace scoped thread spawning with 
`tokio::spawn` and do a little dance to make sure everything finished in the right order. We also ended up
wrapping our memory mapped file in an `Arc` to avoid lifetime issues.

But the result is surprising. It completes a billion rows in `8.2967` seconds. So that's even faster than
the threaded version! Maybe `async` isn't so bad after all?

## It's a Trick!

We have very carefully avoided Tokio's foot-guns (and my goodness, there are many) while playing to Tokio's strengths.

* We aren't using Tokio for disk I/O at all. We're still memory mapping the file and sharing the bytes as an `Arc`.
* We very carefully didn't oversubscribe the system - we never have more tasks than we have cores.
* We used Tokio channels.
  * Tokio channels can sometimes avoid a CPU context switch altogether, because Tokio schedules *tasks* when an `await` point comes along.
  * So every 1000 rows, we send a message---and if the receiver is ready, it might immediately process the work.

So let's see what happens if we doubly allocate our CPU!

```rust
let num_cpus = available_parallelism()?.get() * 2;
```

The overall execution time changes to `8.3 seconds`. In other words, it didn't change very much at all--within the margin of error
of our measurements. Maybe use even more tasks?

```rust
let num_cpus = available_parallelism()?.get() * 10;
```

That measured at `8.27` seconds---in other words, very little appreciable difference.

## So What's the Point?

We're calling `await` often enough---and our tasks are running fast enough---that we're not committing the cardinal sin
of async programming: blocking the thread. We're not doing any disk I/O as far as Tokio is concerned. We're not
hogging a CPU core with a long-running task. We're not oversubscribing the system.

So you *can* use async for this type of work! But you have to be very careful. And you have to be very aware of what
Tokio is doing under the hood. And you have to be very aware of what your tasks are doing.