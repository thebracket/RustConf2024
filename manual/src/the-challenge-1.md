# The Challenge

Given this *giant* list of data, we have to read it in. Each weather station needs to generate a minimum,
maximum and mean average temperature. Then we need to output it as a semicolon separated list:

```csv
Hendon;32.6;67.8;498.6316
Ibagué;-6.1;21.7;59
Lugoj;38.6;58.5;467.07693
```

> And on, and on, and on...

I like this challenge because it's a good mix of data processing and I/O, and you can approach it in layers: starting
with a simple solution, and working up to something really fast.

It's also representative of a lot of real-world problems. You have a big list of data, and you need to process it.
That's a remarkable portion of the low-level programming world!