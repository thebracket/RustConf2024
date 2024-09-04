# Memory Map the File

If you've been in the UNIX world for a while, you've probably run into the
`mmap` system call. Windows and Mac have similar functionality built in.

Memory mapping a file means that you ask the operating system to map the contents
of a file to a region of memory. Typically, this uses the same functionality as
swap files. The operating system will "page in" whole chunks as needed, and "page out"
chunks when they aren't required. Best of all, the OS does that for you.

Even better: you can now treat your file like a huge, contiguous slice of
bytes. Your whole file is now of the type `&[u8]`.

## Dependencies

We're going to use the `memmap` crate, which provides an interface to various
operating systems' memory mapping functionality.

```toml
[workspace.dependencies]
anyhow = "1.0.44"
rustc-hash = "2.0.0"
memmap = "0.7.0"
```

## How to memory map a file?

Memory mapping a file is quite straightforward:

```rust
let file = File::open("../data_builder/measurements.txt")?;
let memory_map = unsafe { memmap::Mmap::map(&file)? }; // It's now a big sea of bytes!
```

> Uh oh! We just used the dreaded `unsafe` keyword. Memory mapping is pretty safe,
> but it's a *system call*. It's defined by the operating system, and `memmap` is
> just providing a convenient wrapper. So you're passing outside the domain that
> Rust can verify---which is one definition of `unsafe`.
> 
> `Unsafe` doesn't mean "shout Leeroy Jenkins and charge" (although it can). It means
> "I'm doing something that Rust can't verify is safe, but I'm pretty sure it is."
> 
> It also helpfully flags to other developers that they should be careful around this
> code.
> 
> Lastly, it's a great way to attract random PRs on GitHub.
