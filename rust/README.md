# Info
- using rust to calculate mandelbrot fractal

# usage
- build: `cargo build --release`
- run: `target/release/mandelbrot --xres 1000 -yres 1000`
  - generates default file:`test.png`
# Options

```
      --xres <XRES>          resolution in x [default: 1000]
      --yres <YRES>          resolution in y [default: 1000]
      --x-min <X_MIN>        minimum of x [default: -1.5]
      --x-max <X_MAX>        maximum of x [default: 0.5]
      --y-min <Y_MIN>        minimum of y [default: -1]
      --y-max <Y_MAX>        maximum of y [default: 1]
  -t, --threads <THREADS>    number of threads [default: 8]
  -f, --filepath <filepath>  filepath
  -h, --help                 Print help
  -V, --version              Print version
```

# History
- 20230214 use of threads
- 20230213 first version to output PNG file
