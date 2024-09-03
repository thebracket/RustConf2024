# Memory Map the File

So we've run into a bit of a wall: you can't readily multitask a line reader,
because it iterates forward - skipping around isn't going to work.

Let's take a page out of ripgrep's book and memory map the file. This allows us
to treat the file like a giant array of bytes, and skip around it - without having
to read it all into memory or be really careful with seek order.

```
File & hash time: 0.061s
Calculate time:   0.001s
Print time:       0.721s
TOTAL:            0.783s

```

Or with 1 billion rows:

```
File & hash time: 52.246s
Calculate time:   0.001s
Print time:       0.727s
TOTAL:            52.974s
```

So we're down from `86.433s` to `52.974s` without using more CPUs!