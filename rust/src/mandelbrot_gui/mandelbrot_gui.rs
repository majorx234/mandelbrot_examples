use eframe::egui;
use eframe::egui::{lerp, Color32, Rgba, TextureHandle};
use egui::plot::{GridInput, GridMark};
use egui::*;
use plot::{
    Arrows, Bar, BarChart, CoordinatesFormatter, Corner, HLine, Legend, Line, LineStyle,
    MarkerShape, Plot, PlotImage, Points, Polygon, Text, VLine,
};

struct MandelbrotData {
    pub tex_mngr: TextureManager,
    pub texture_id: Option<(egui::Vec2, egui::TextureId)>,
}

impl Default for MandelbrotData {
    fn default() -> Self {
        Self {
            tex_mngr: TextureManager(vec![Color32::from_rgb(255, 255, 255); 1024 * 512], None),
            texture_id: None,
        }
    }
}

impl MandelbrotData {
    fn ui(&mut self, ui: &mut Ui, mandelbrot_data: Vec<Vec<f32>>) {
        self.set_values(ui.ctx(), mandelbrot_data);

        if let Some((size, texture_id)) = self.texture_id {
            ui.add(egui::Image::new(texture_id, size));
            ui.ctx().request_repaint();
        }
        // ui.horizontal(|ui| {});
        // self.bar_plot(ui);
    }

    fn set_values(&mut self, ctx: &egui::Context, specs: Vec<Vec<f32>>) {}
}

//#[derive(Default)]
struct TextureManager(Vec<egui::epaint::Color32>, Option<TextureHandle>);

impl TextureManager {
    pub fn update_spectrogram_texture(
        &mut self,
        ctx: &egui::Context,
        specs: Vec<Vec<u8>>,
        width: usize,
        height: usize,
    ) {
        let mut mandelbrot_img = specs
            .into_iter()
            .flatten()
            .map(|x| egui::epaint::Color32::from_gray(x))
            .collect::<Vec<Color32>>();
        self.0 = mandelbrot_img;
        let pixels: Vec<egui::epaint::Color32> = self.0.clone();
        self.1 = Some(ctx.load_texture(
            "color_test_gradient",
            egui::ColorImage {
                size: [width, height],
                pixels,
            },
        ));
    }
}

pub struct MandelbrotGui {
    mandelbrot: MandelbrotData,
}

impl MandelbrotGui {
    pub fn new() -> Self {
        Self {
            mandelbrot: MandelbrotData::default(),
        }
    }
}
impl Default for MandelbrotGui {
    fn default() -> Self {
        Self {
            mandelbrot: MandelbrotData::default(),
        }
    }
}
impl eframe::App for MandelbrotGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let test_vec = Vec::new();
            self.mandelbrot.ui(ui, test_vec);
        });
    }
}
