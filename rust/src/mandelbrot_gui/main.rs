mod mandelbrot_gui;
use eframe;
use mandelbrot_gui::MandelbrotGui;
use mandelbrot_utils::calc::calculate_mandelbrot;

fn main() {
    let mut my_mandelbrot: Vec<u8> = vec![0; 800 * 800];
    calculate_mandelbrot(&mut my_mandelbrot, -1.5, 0.5, -1.0, 1.0, 800, 800);
    let mandelbrot_app = MandelbrotGui::new(my_mandelbrot);
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "MandelbrotGui",
        options,
        Box::new(|_cc| Box::new(mandelbrot_app)),
    );
}
