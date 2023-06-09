mod mandelbrot_gui;
use mandelbrot_gui::MandelbrotGui;

use eframe;

fn main() {
    let mut mandelbrot_app = MandelbrotGui::new();
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "MandelbrotGui",
        options,
        Box::new(|_cc| Box::new(mandelbrot_app)),
    );
}
