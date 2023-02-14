use clap::{Arg, Command, Parser};
use mandelbrot_utils::calc::calculate_mandelbrot;
use mandelbrot_utils::image::write_png;

// mandelbrot caclulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// window size - how many samples to display
    #[arg(short, long, default_value_t = 1000)]
    pub xres: usize,

    /// how many samples to display in one line (max 1024)
    #[arg(short, long, default_value_t = 1000)]
    pub yres: usize,
}

fn main() {
    let args = Args::parse();
    let my_mandelbrot = calculate_mandelbrot(-1.5, 0.5, -1.0, 1.0, args.xres, args.yres);
    write_png("test.png", &my_mandelbrot, (args.xres, args.yres));
}
