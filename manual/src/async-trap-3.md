# Sending Results to a Server

It's much more common to run into this pattern:

```
(Data Arrives) -> (CPU bound processing) -> (Data Sent to Server)
```

And this is where people start to get confused. 

* The data arrival may be naturally async (or not)---some form of notification that there's work to do.
* The CPU bound processing is naturally synchronous.
* The data sent to the server is naturally async.

In the 1BR challenge, you can get away with having an async channel, doing the processing async, and having
your submission task be something like:

```rust
while let Ok(msg) = rx.recv().await {
    // Send the message to the server
    reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&msg)
        .send()
        .await?;
}
```

But for this to work in a truly CPU demanding environment, you risk the CPU-bound tasks blocking heavily. That
leaves you with some alternatives:

* Use `ureq` and don't go async at all. That's often the right choice.
* Use `yield_now` occasionally in your CPU bound code to ensure that the other tasks get to run.
* Use `tokio::task::spawn_blocking` to run the CPU bound code in a separate thread. And now you're worrying about too many threads again.
* Carefully quarantine the async code.

Quarantine is often the best answer:

1. Create a Tokio channel for submission.
2. Use `std::thread` to spawn your async code.
   3. Call `block_on` with a *single threaded Tokio* in the thread and poll the channel.
4. Your CPU bound workload runs as threads and submits results to the channel.
   5. But wait? Isn't it an async channel? Yes, but Tokio's async channel has a function called `blocking_send` for exactly this purpose.

Now you've got the best of both worlds.