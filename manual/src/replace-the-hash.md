# Replace the Hash

We add a dependency:

```toml
rustc-hash = "2.0.0"
```

And replace all of the `HashMap` with `FxHashMap`. Curiously, we have to replace `HashMap::new()` with `FxHashMap::default()`.

So let's give that a spin with 1 million rows:

```
File & hash time: 0.113s
Calculate time:   0.006s
Print time:       0.758s
TOTAL:            0.877s
```

File & hash time has improved from `0.124s` to `0.113s`. Not a huge improvement, but it's something.

With 1 billion rows:

```
File & hash time: 105.491s
Calculate time:   1.575s
Print time:       0.850s
TOTAL:            107.915s
```

That's improved from `119.590s` to `107.915s`. So the hasher probably *isn't* the problem---it helped,
but it didn't solve the performance issue.