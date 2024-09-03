# Accumulate

Since we just want a min/max/mean - we only need to *accumulate* some data rather
than storing every entry. We no longer have to allocate a vector for every station,
nor expand it as we read more data.

```
File & hash time: 0.096s
Calculate time:   0.001s
Print time:       0.723s
TOTAL:            0.820s
```

So our total time has slightly improved from `0.877s` to `0.820s`. Not a huge improvement, but it's something.

Let's try it with 1 billion rows:

```
File & hash time: 85.700s
Calculate time:   0.001s
Print time:       0.732s
TOTAL:            86.433s

```

That's an improvement from `107.915s`.