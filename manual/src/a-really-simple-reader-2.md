# Dependencies

We're only using the one dependency: `anyhow`. You don't even need that one, but it makes error handling a lot easier.

```toml
anyhow = "1.0.44"
```

Notice how in the repo it says:

```toml
[dependencies]
anyhow = {  workspace = true }
```

I've defined workspace dependencies in the workspace's `Cargo.toml`, so compilation will be shared between
all of them. This saves some compilation time and disk space - maybe even enough to justify 15 Gb of
test data!