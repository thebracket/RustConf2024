# Introduction

Most modern PCs have lots of CPU cores --- and a lot of programs only use one of them. Maybe two! Why is this?

* Concurrent programming is *hard*. You have to worry about data races, deadlocks, poisoning.
* Concurrent programming is *hard to reason about*. We naturally think about doing one thing at a time, while multiple threads are executing at the same time. So suddenly you have to worry about the effective order of operations, while multiple parts are moving at once.
* Many languages make it *really easy* to shoot yourself in the foot. Concurrently.

Rust promises *fearless concurrency* - so let's put that to the test!

We're going to focus on a problem applicable to the real-world: parsing lots of data and generating some statistics.
This is the kind of number crunching that makes Rust look good!

![img_2.png](img_2.png)