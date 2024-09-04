# Results

1 million rows:

```
File & hash time: 0.060s
Calculate time:   0.001s
Print time:       0.660s
TOTAL:            0.722s
```

1 billion rows:

```
File & hash time: 55.066s
Calculate time:   0.001s
Print time:       0.681s
TOTAL:            55.748s
```

We've gone from 89 seconds to 55 seconds just by using `mmap`! That's a remarkable improvement for
very little code. We're still only using 1 CPU. Let's fix that.