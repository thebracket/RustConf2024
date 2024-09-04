# Results

For 1 million rows:

```
File & hash time: 0.057s
Calculate time:   0.001s
Print time:       0.738s
TOTAL:            0.796s
```

That's really good! How about a billion rows?

```
File & hash time: 21.093s
Calculate time:   0.001s
Print time:       0.711s
TOTAL:            21.805s
```

We're down from 55 seconds to 21 seconds. That's a really impressive improvement! It's, however, a linear
improvement with the number of cores we have.