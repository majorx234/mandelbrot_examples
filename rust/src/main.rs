use clap::Parser;
use crossbeam;
use mandelbrot_utils::calc::calculate_mandelbrot;
use mandelbrot_utils::image::write_png;
use std::time::SystemTime;

// mandelbrot caclulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// resolution in x
    #[arg(long, default_value_t = 1000)]
    pub xres: usize,

    /// resolution in y
    #[arg(long, default_value_t = 1000)]
    pub yres: usize,

    /// minimum of x
    #[arg(long, default_value_t = -1.5)]
    pub x_min: f64,

    /// maximum of x
    #[arg(long, default_value_t = 0.5)]
    pub x_max: f64,

    /// minimum of y
    #[arg(long, default_value_t = -1.0)]
    pub y_min: f64,

    /// maximum of y
    #[arg(long, default_value_t = 1.0)]
    pub y_max: f64,

    /// number of threads
    #[arg(short, long, default_value_t = 8)]
    pub threads: usize,

    /// filepath
    #[arg(short, long, value_name = "filepath")]
    pub filepath: Option<String>,
}

fn main() {
    let args = Args::parse();
    let threads = args.threads;
    assert_eq!(args.yres % threads, 0);
    let filepath = if let Some(filepath) = args.filepath {
        filepath
    } else {
        "test.png".to_string()
    };
    let time_before = SystemTime::now();
    let mut my_mandelbrot: Vec<u8> = vec![0; args.xres * args.yres];
    let pixels_per_thread = args.xres * args.yres / threads;
    let y_min = args.y_min; //-1.0;
    let y_max = args.y_max; //1.0;
    let x_min = args.x_min; //-1.5;
    let x_max = args.x_max; //0.5;
    let xres = args.xres;
    let yres = args.yres;

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
            let wg_clone = wg.clone();
            let thread = spawner.spawn(move |_| {
                calculate_mandelbrot(
                    mandelbrot,
                    x_min,
                    x_max,
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
    write_png(&filepath, &my_mandelbrot, (args.xres, args.yres)).unwrap();
}
