# A Really Simple Reader

So let's start with a really simple, step-by-step reader. The code for this is in `code/simple_line_reader`.

We're going to build a very simple reader that breaks all the steps down into individual functions,
and times them. This will give us a good idea of how long each step takes, and give us a handle
on where we might want to focus our efforts.

The steps:

1. Read the file line by line.
2. Break readings out into a `HashMap`, keyed on the station name and storing readings.
3. Iterate through each station's readings and calculate the minimum, maximum, and average.
4. Print out the results in the desired format.

## But First: Some Macro Magic

I got really sick of typing `let start = Instant::now; ... ; let duration = start.elapsed();` over and over again.
So there's a handy macro in here:

```rust
macro_rules! time_it {
    ($block:block, $timer:expr) => {
        {
            let start = std::time::Instant::now();
            let result = $block;
            let elapsed = start.elapsed().as_secs_f32();
            $timer = elapsed;
            result
        }
    };
}
```

> Don't macros look like a cat walked on the keyboard?

Macros extend *syntax* and are expanded out to actual code during pre-processing. They're a powerful tool,
but easy to overuse.

So the inputs are: a block of code, and variable in which to store the execution time. A timer is started,
the block of code executed, and the elapsed time (in seconds, as a float) stored in the variable. This lets us
focus on the meat of the code we're writing. For example:

```rust
let file_reader_time;
let rows = time_it!({
        read_file()?
    }, file_reader_time);
```