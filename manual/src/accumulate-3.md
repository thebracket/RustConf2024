# Results

With a million rows:

```
File & hash time: 0.095s
Calculate time:   0.001s
Print time:       0.699s
TOTAL:            0.795s
```

And with a billion rows:

```
File & hash time: 89.153s
Calculate time:   0.002s
Print time:       0.683s
TOTAL:            89.837s
```

We're down from 109 seconds to 89 seconds. That's a pretty decent improvement! If you watch 
`htop` or similar, we're using a much more reasonable amount of memory now, too!

# Next

So we haven't dealt with concurrency at all yet. That's because file reading really isn't very
amenable to concurrency: file readers work their way down the file in order, and without loading
all the data into memory it's *hard* to parallelize. Sticking Rayon `par_iter` onto a file reader
isn't as useful as you might think; you could easily replace some of the other iterators with
`par_iter`---but the speed-up will be quite limited.

Wouldn't it be nice if we could see the whole file at once, without reading it all into memory?
