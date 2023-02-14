use clap::Parser;
use crossbeam;
use mandelbrot_utils::calc::calculate_mandelbrot;
use mandelbrot_utils::image::write_png;
use std::time::SystemTime;

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
    let threads = 8;
    let args = Args::parse();
    let time_before = SystemTime::now();
    let mut my_mandelbrot: Vec<u8> = vec![0; args.xres * args.yres];
    let pixels_per_thread = args.xres * args.yres / threads;
    let y_min = -1.0;
    let y_max = 1.0;
    let delta_y = (y_max - y_min) / threads as f64;
    let y_values = (0..threads)
        .map(|x| x as f64 * delta_y + y_min)
        .collect::<Vec<f64>>();
    let mandelbrots = my_mandelbrot
        .chunks_mut(pixels_per_thread)
        .collect::<Vec<&mut [u8]>>();

    crossbeam::scope(|spawner| {
        let wg = crossbeam::sync::WaitGroup::new();
        for (i, (mandelbrot, y_value)) in mandelbrots.into_iter().zip(y_values).enumerate() {
            let xres = args.xres.clone();
            let yres = args.yres.clone();
            let wg_clone = wg.clone();
            let thread = spawner.spawn(move |_| {
                calculate_mandelbrot(
                    mandelbrot,
                    -1.5,
                    0.5,
                    y_value,
                    y_value + delta_y,
                    xres,
                    yres / threads,
                );
                drop(wg_clone);
            });
        }
        wg.wait();
    });

    let time_after = SystemTime::now();
    let time_difference = time_after.duration_since(time_before).unwrap().as_millis();
    print!("caclulation take {} ms", time_difference);
    write_png("test.png", &my_mandelbrot, (args.xres, args.yres)).unwrap();
}
