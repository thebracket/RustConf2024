# It's a trap!

![img_10.png](img_10.png)

Async isn't really a trap, but it gets a bad name:

* Async tends to "infect" the rest of your code. 
  * Async "colours" functions, such that only async code can call other async code.
  * Async sounds awesome, but it's really not well suited for CPU-bound work in many cases.
* Tokio mitigates this a bit by using a thread-per-core model.
  * Until it doesn't.
  * Calling file I/O functions from async code launches a blocking thread (via `spawn_blocking`).
    * Suddenly you have an indeterminate number of threads.
    * Data is being copied to keep the async system happy.

In other words: async isn't great for CPU-bound work, and can struggle with disk-bound work
unless you are using `io_uring` or Windows completion ports. And suddenly your code isn't
portable.

Ouch.