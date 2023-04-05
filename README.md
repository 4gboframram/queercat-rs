# queercat-rs
Pride flags in the terminal, but rusty :3 
## Usage
```
Concatenate FILE(s), or standard input, to standard output. With no FILE, or when FILE is -, read standard input

Usage: queercat [OPTIONS] [FILES]...

Arguments:
  [FILES]...  The files to read

Options:
  -f, --flag <FLAG>
          The builtin flag to use if a custom flag pattern is not specified [default: rainbow] [possible values: rainbow, transgender, non-binary, lesbian, gay, pansexual, bisexual, gender-fluid, asexual, unlabeled, aromantic, aroace]
  -s, --stripes <STRIPES>
          Stripes for the flag entered as hexadecimal numbers
  -a, --ansi-codes <ANSI_CODES>
          Ansi codes for the flag written as decimal numbers
      --factor <FACTOR>
          [default: 4]
  -b, --24bit
          Whether to use 24 bit RGB color. This may be slower and incompatible with some terminals, but it looks amazing
  -o, --offset <OFFSET>
          Offset of the start of the flag [default: 0]
  -z, --horizontal-frequency <HORIZONTAL_FREQUENCY>
          Horizontal rainbow frequency [default: 0.1]
  -v, --vertical-frequency <VERTICAL_FREQUENCY>
          Vertical rainbow frequency [default: 0.23]
  -h, --help
          Print help
  -V, --version
          Print version
```

**NOTE** for `-a` and `-s`, you have to write the flag multiple times
## Installation
- Release binaries are provided for x86-64 windows, mac, and linux
- You can build the cli from source with `cargo install --path ./queercat`

## Motivation
I was working on a pride-themed shell and I wanted to have [queercat](https://github.com/Elsa002/queercat) as a builtin command and to use it as a library for internals. 

I then looked at the code and noticed many flaws that hindered performance such as:
- Printing the color code even when the color didn't change*
- Using a 2 `while` loops instead of `fmod` to put theta in the range of `[0, 2pi]`
- Using a `for` loop instead of division to determine the index of the next stripe color
- `lrintf`... Just why

*the funny thing is that for ansi colors, there was an attempt to cache, but assigned to a local instead of writing to a pointer
## Bug Reporting / Feature Requests
- You can report any bugs you find or any features you would like added at the [github repo](https://github.com/4gboframram/queercat-rs/issues)
## Acknowledgements
Huge thanks to Elsa002 for making the [original queercat](https://github.com/Elsa002/queercat). Without the original queercat, there would be no rust rewrite.
I would not have been able to make a rewrite nor write this code from scratch without without the original code. 

As a nod to the original project, this code is released under the same license: the `Unlicense`, and is dedicated to the public domain.
## Changelog
- **v0.1.1**:
  - Updated README
  - `-o`, `-z`, and `-v` flags now work
  - Stdout now buffers when input is not piped
  - Normalized all angles to between 0 and 1
  - Converted most floating-point operations to fixed-point
  - Changed the default horizontal and vertical frequencies to reflect the original
  - Lowered the minimum balance threshold so flags look cooler
  - **tldr;** things work more consistently and they work slightly faster
  - Removed the proc macro crate because it was unnecessary to begin with, and even more unnecessary with the new changes
- **v0.1.0**: initial commit
## Todo
- Publish to `crates.io` when version `1.0.0` is released
- Add screenshots
- Optimize the grapheme iterator, as profiling says that it takes most of the cpu time (other than io)
- Add more queer easter eggs
- Maybe interpolate colors in hsv color space instead of rgb
- Maybe add a config file for custom flags that can be loaded by name later
